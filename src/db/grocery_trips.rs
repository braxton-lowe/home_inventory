use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{CreateGroceryTrip, FoodItem, GroceryTrip, GroceryTripWithItems, ListParams, UpdateGroceryTrip};

use super::trip_items::link_items_to_trip;

pub async fn list_grocery_trips(pool: &PgPool, params: &ListParams) -> AppResult<Vec<GroceryTrip>> {
    let sort_col = params.sort_column(&["trip_date", "store_name", "total_spent"], "trip_date");
    let sort_dir = params.sort_direction_or("DESC");
    let limit = params.limit_or(50);
    let offset = params.offset_or(0);
    let search = params.search_filter();

    let where_clause = if search.is_some() {
        "WHERE store_name ILIKE $3 OR COALESCE(notes, '') ILIKE $3"
    } else {
        ""
    };

    let query = format!(
        "SELECT id, trip_date, store_name, total_spent, notes, created_at, updated_at FROM grocery_trips {} ORDER BY {} {} LIMIT $1 OFFSET $2",
        where_clause, sort_col, sort_dir
    );

    let mut q = sqlx::query_as::<_, GroceryTrip>(&query)
        .bind(limit)
        .bind(offset);
    if let Some(ref term) = search {
        q = q.bind(term);
    }
    let trips = q.fetch_all(pool).await?;

    Ok(trips)
}

pub async fn get_grocery_trip(pool: &PgPool, id: Uuid) -> AppResult<GroceryTripWithItems> {
    let trip = sqlx::query_as!(
        GroceryTrip,
        r#"
        SELECT id, trip_date, store_name, total_spent, notes, created_at, updated_at
        FROM grocery_trips
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    let items = sqlx::query_as!(
        FoodItem,
        r#"
        SELECT
            fi.id,
            fi.type as "item_type!",
            fi.brand,
            fi.name,
            fi.quantity,
            fi.unit,
            fi.price,
            fi.expiration_date,
            fi.purchase_date,
            fi.notes,
            fi.location_id,
            fi.active,
            fi.consumed_at,
            fi.created_at,
            fi.updated_at
        FROM food_items fi
        INNER JOIN trip_items ti ON fi.id = ti.food_item_id
        WHERE ti.trip_id = $1
        ORDER BY fi.name
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(GroceryTripWithItems { trip, items })
}

pub async fn create_grocery_trip(pool: &PgPool, data: CreateGroceryTrip) -> AppResult<GroceryTrip> {
    let mut tx = pool.begin().await?;

    let trip = sqlx::query_as!(
        GroceryTrip,
        r#"
        INSERT INTO grocery_trips (trip_date, store_name, total_spent, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING id, trip_date, store_name, total_spent, notes, created_at, updated_at
        "#,
        data.trip_date,
        data.store_name,
        data.total_spent,
        data.notes
    )
    .fetch_one(&mut *tx)
    .await?;

    if let Some(item_ids) = data.item_ids {
        if !item_ids.is_empty() {
            for item_id in item_ids {
                sqlx::query!(
                    r#"
                    INSERT INTO trip_items (trip_id, food_item_id)
                    VALUES ($1, $2)
                    "#,
                    trip.id,
                    item_id
                )
                .execute(&mut *tx)
                .await?;
            }
        }
    }

    tx.commit().await?;

    Ok(trip)
}

pub async fn update_grocery_trip(
    pool: &PgPool,
    id: Uuid,
    data: UpdateGroceryTrip,
) -> AppResult<GroceryTrip> {
    let trip = sqlx::query_as!(
        GroceryTrip,
        r#"
        UPDATE grocery_trips
        SET trip_date = $1, store_name = $2, total_spent = $3, notes = $4
        WHERE id = $5
        RETURNING id, trip_date, store_name, total_spent, notes, created_at, updated_at
        "#,
        data.trip_date,
        data.store_name,
        data.total_spent,
        data.notes,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(trip)
}

pub async fn delete_grocery_trip(pool: &PgPool, id: Uuid) -> AppResult<()> {
    let result = sqlx::query!("DELETE FROM grocery_trips WHERE id = $1", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound(format!(
            "Grocery trip with id {} not found",
            id
        )));
    }

    Ok(())
}
