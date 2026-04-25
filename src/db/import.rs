use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{ImportResult, ItemCsvRow, MealCsvRow, TripCsvRow};

async fn resolve_locations(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    location_names: &HashSet<String>,
) -> AppResult<(HashMap<String, Uuid>, usize)> {
    let mut map = HashMap::new();
    let mut created = 0;

    for name in location_names {
        let row = sqlx::query_scalar!(
            "SELECT id FROM locations WHERE LOWER(name) = LOWER($1)",
            name
        )
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(id) = row {
            map.insert(name.clone(), id);
        } else {
            let id = sqlx::query_scalar!(
                "INSERT INTO locations (name) VALUES ($1) RETURNING id",
                name
            )
            .fetch_one(&mut **tx)
            .await?;

            map.insert(name.clone(), id);
            created += 1;
        }
    }

    Ok((map, created))
}

/// Upserts a food item. Returns (id, was_created).
/// Uses the (name, brand, type, location_id, purchase_date) natural key.
async fn upsert_food_item_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_type: &str,
    brand: &Option<String>,
    name: &str,
    quantity: Decimal,
    unit: &str,
    price: &Option<Decimal>,
    expiration_date: &Option<NaiveDate>,
    purchase_date: NaiveDate,
    notes: &Option<String>,
    location_id: Uuid,
) -> AppResult<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"
        INSERT INTO food_items (
            type, brand, name, quantity, unit, price,
            expiration_date, purchase_date, notes, location_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (name, COALESCE(brand, ''), type, location_id, purchase_date)
        DO UPDATE SET
            quantity = EXCLUDED.quantity,
            unit = EXCLUDED.unit,
            price = EXCLUDED.price,
            expiration_date = EXCLUDED.expiration_date,
            notes = EXCLUDED.notes,
            active = true,
            consumed_at = NULL,
            updated_at = NOW()
        RETURNING id, (xmax = 0) as "was_created!"
        "#,
        item_type,
        brand.as_deref(),
        name,
        quantity,
        unit,
        *price,
        *expiration_date,
        purchase_date,
        notes.as_deref(),
        location_id
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok((row.id, row.was_created))
}

pub async fn process_items_import(pool: &PgPool, rows: Vec<ItemCsvRow>) -> AppResult<ImportResult> {
    let row_count = rows.len();

    let location_names: HashSet<String> = rows.iter().map(|r| r.location.clone()).collect();

    let mut tx = pool.begin().await?;

    let (location_map, locations_created) = resolve_locations(&mut tx, &location_names).await?;

    let mut items_created = 0;
    let mut items_updated = 0;
    for (i, row) in rows.iter().enumerate() {
        let location_id = location_map.get(&row.location).ok_or_else(|| {
            AppError::InternalError(format!("Row {}: location '{}' not resolved", i + 1, row.location))
        })?;

        let purchase_date = row.purchase_date.unwrap_or_else(|| chrono::Utc::now().date_naive());

        let (_id, was_created) = upsert_food_item_in_tx(
            &mut tx,
            &row.item_type,
            &row.brand,
            &row.name,
            row.quantity,
            &row.unit,
            &row.price,
            &row.expiration_date,
            purchase_date,
            &row.notes,
            *location_id,
        )
        .await?;

        if was_created {
            items_created += 1;
        } else {
            items_updated += 1;
        }
    }

    tx.commit().await?;

    Ok(ImportResult {
        rows_processed: row_count,
        locations_created: Some(locations_created),
        food_items_created: Some(items_created),
        food_items_updated: Some(items_updated),
        trips_created: None,
        trips_updated: None,
        trip_items_linked: None,
        meals_created: None,
        meals_updated: None,
    })
}

pub async fn process_trip_import(pool: &PgPool, rows: Vec<TripCsvRow>) -> AppResult<ImportResult> {
    let row_count = rows.len();

    let location_names: HashSet<String> = rows.iter().map(|r| r.location.clone()).collect();

    let mut tx = pool.begin().await?;

    let (location_map, locations_created) = resolve_locations(&mut tx, &location_names).await?;

    // Group rows by (trip_date, store_name)
    let mut trip_groups: Vec<(NaiveDate, String, Option<Decimal>, Option<String>, Vec<&TripCsvRow>)> = Vec::new();

    for row in &rows {
        let key = (row.trip_date, row.store_name.clone());
        if let Some(group) = trip_groups.iter_mut().find(|g| g.0 == key.0 && g.1 == key.1) {
            group.4.push(row);
        } else {
            trip_groups.push((key.0, key.1, row.total_spent, row.trip_notes.clone(), vec![row]));
        }
    }

    let mut trips_created = 0;
    let mut trips_updated = 0;
    let mut items_created = 0;
    let mut items_updated = 0;
    let mut items_linked = 0;

    for (trip_date, store_name, total_spent, trip_notes, group_rows) in &trip_groups {
        let trip_row = sqlx::query!(
            r#"
            INSERT INTO grocery_trips (trip_date, store_name, total_spent, notes)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (trip_date, store_name)
            DO UPDATE SET
                total_spent = EXCLUDED.total_spent,
                notes = EXCLUDED.notes,
                updated_at = NOW()
            RETURNING id, (xmax = 0) as "was_created!"
            "#,
            *trip_date,
            store_name,
            *total_spent,
            trip_notes.as_deref()
        )
        .fetch_one(&mut *tx)
        .await?;

        if trip_row.was_created {
            trips_created += 1;
        } else {
            trips_updated += 1;
        }

        for row in group_rows {
            let location_id = location_map.get(&row.location).ok_or_else(|| {
                AppError::InternalError(format!(
                    "Row: location '{}' not resolved",
                    row.location
                ))
            })?;

            let purchase_date = row.purchase_date.unwrap_or(row.trip_date);

            let (food_item_id, was_created) = upsert_food_item_in_tx(
                &mut tx,
                &row.item_type,
                &row.brand,
                &row.name,
                row.quantity,
                &row.unit,
                &row.price,
                &row.expiration_date,
                purchase_date,
                &row.item_notes,
                *location_id,
            )
            .await?;

            if was_created {
                items_created += 1;
            } else {
                items_updated += 1;
            }

            sqlx::query!(
                r#"
                INSERT INTO trip_items (trip_id, food_item_id)
                VALUES ($1, $2)
                ON CONFLICT (trip_id, food_item_id) DO NOTHING
                "#,
                trip_row.id,
                food_item_id
            )
            .execute(&mut *tx)
            .await?;

            items_linked += 1;
        }
    }

    tx.commit().await?;

    Ok(ImportResult {
        rows_processed: row_count,
        locations_created: Some(locations_created),
        food_items_created: Some(items_created),
        food_items_updated: Some(items_updated),
        trips_created: Some(trips_created),
        trips_updated: Some(trips_updated),
        trip_items_linked: Some(items_linked),
        meals_created: None,
        meals_updated: None,
    })
}

pub async fn process_meals_import(pool: &PgPool, rows: Vec<MealCsvRow>) -> AppResult<ImportResult> {
    let row_count = rows.len();

    let mut tx = pool.begin().await?;

    let mut meals_created = 0;
    let mut meals_updated = 0;

    for row in &rows {
        let meal_row = sqlx::query!(
            r#"
            INSERT INTO meals (name, made_on, servings, last_eaten, rating, ingredients, recipe_link)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (name, made_on)
            DO UPDATE SET
                servings = EXCLUDED.servings,
                last_eaten = EXCLUDED.last_eaten,
                rating = EXCLUDED.rating,
                ingredients = EXCLUDED.ingredients,
                recipe_link = EXCLUDED.recipe_link,
                updated_at = NOW()
            RETURNING id, (xmax = 0) as "was_created!"
            "#,
            row.name,
            row.made_on,
            row.servings,
            row.last_eaten,
            row.rating,
            row.ingredients,
            row.recipe_link
        )
        .fetch_one(&mut *tx)
        .await?;

        if meal_row.was_created {
            meals_created += 1;
        } else {
            meals_updated += 1;
        }
    }

    tx.commit().await?;

    Ok(ImportResult {
        rows_processed: row_count,
        locations_created: None,
        food_items_created: None,
        food_items_updated: None,
        trips_created: None,
        trips_updated: None,
        trip_items_linked: None,
        meals_created: Some(meals_created),
        meals_updated: Some(meals_updated),
    })
}
