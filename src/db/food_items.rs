use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{CreateFoodItem, FoodItem, ListParams, UpdateFoodItem};

pub async fn list_food_items(pool: &PgPool, params: &ListParams) -> AppResult<Vec<FoodItem>> {
    let sort_col = params.sort_column(
        &["name", "type", "brand", "quantity", "unit", "price", "purchase_date", "expiration_date"],
        "purchase_date",
    );
    let sort_dir = params.sort_direction_or("DESC");
    let limit = params.limit_or(50);
    let offset = params.offset_or(0);
    let search = params.search_filter();
    let active_filter = params.active_filter();

    let mut conditions: Vec<String> = Vec::new();

    // Active filter (param index depends on whether search is present)
    match active_filter {
        Some(val) => {
            conditions.push(format!("active = {}", val));
        }
        None => {} // show all
    }

    if search.is_some() {
        conditions.push("(name ILIKE $3 OR COALESCE(brand, '') ILIKE $3 OR COALESCE(notes, '') ILIKE $3)".to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let query = format!(
        r#"
        SELECT
            id,
            type AS item_type,
            brand,
            name,
            quantity,
            unit,
            price,
            expiration_date,
            purchase_date,
            notes,
            location_id,
            active,
            consumed_at,
            created_at,
            updated_at
        FROM food_items
        {}
        ORDER BY {} {}
        LIMIT $1 OFFSET $2
        "#,
        where_clause, sort_col, sort_dir
    );

    let mut q = sqlx::query_as::<_, FoodItem>(&query)
        .bind(limit)
        .bind(offset);
    if let Some(ref term) = search {
        q = q.bind(term);
    }
    let items = q.fetch_all(pool).await?;

    Ok(items)
}

pub async fn get_food_item(pool: &PgPool, id: Uuid) -> AppResult<FoodItem> {
    let item = sqlx::query_as!(
        FoodItem,
        r#"
        SELECT
            id,
            type as "item_type!",
            brand,
            name,
            quantity,
            unit,
            price,
            expiration_date,
            purchase_date,
            notes,
            location_id,
            active,
            consumed_at,
            created_at,
            updated_at
        FROM food_items
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn create_food_item(pool: &PgPool, data: CreateFoodItem) -> AppResult<FoodItem> {
    let item = sqlx::query_as!(
        FoodItem,
        r#"
        INSERT INTO food_items (
            type, brand, name, quantity, unit, price,
            expiration_date, purchase_date, notes, location_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING
            id,
            type as "item_type!",
            brand,
            name,
            quantity,
            unit,
            price,
            expiration_date,
            purchase_date,
            notes,
            location_id,
            active,
            consumed_at,
            created_at,
            updated_at
        "#,
        data.item_type,
        data.brand,
        data.name,
        data.quantity,
        data.unit,
        data.price,
        data.expiration_date,
        data.purchase_date,
        data.notes,
        data.location_id
    )
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn update_food_item(
    pool: &PgPool,
    id: Uuid,
    data: UpdateFoodItem,
) -> AppResult<FoodItem> {
    let item = sqlx::query_as!(
        FoodItem,
        r#"
        UPDATE food_items
        SET
            type = $1,
            brand = $2,
            name = $3,
            quantity = $4,
            unit = $5,
            price = $6,
            expiration_date = $7,
            purchase_date = $8,
            notes = $9,
            location_id = $10
        WHERE id = $11
        RETURNING
            id,
            type as "item_type!",
            brand,
            name,
            quantity,
            unit,
            price,
            expiration_date,
            purchase_date,
            notes,
            location_id,
            active,
            consumed_at,
            created_at,
            updated_at
        "#,
        data.item_type,
        data.brand,
        data.name,
        data.quantity,
        data.unit,
        data.price,
        data.expiration_date,
        data.purchase_date,
        data.notes,
        data.location_id,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn autocomplete_food_items(pool: &PgPool, query: &str) -> AppResult<Vec<FoodItem>> {
    let pattern = format!("%{}%", query.to_lowercase());
    let items = sqlx::query_as::<_, FoodItem>(
        r#"
        SELECT DISTINCT ON (LOWER(name))
            id,
            type AS item_type,
            brand,
            name,
            quantity,
            unit,
            price,
            expiration_date,
            purchase_date,
            notes,
            location_id,
            active,
            consumed_at,
            created_at,
            updated_at
        FROM food_items
        WHERE LOWER(name) LIKE $1
        ORDER BY LOWER(name), created_at DESC
        LIMIT 10
        "#,
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn deactivate_food_item(pool: &PgPool, id: Uuid) -> AppResult<()> {
    let result = sqlx::query!(
        "UPDATE food_items SET active = false, consumed_at = NOW() WHERE id = $1 AND active = true",
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound(format!(
            "Food item with id {} not found or already inactive",
            id
        )));
    }

    Ok(())
}

pub async fn bulk_deactivate_food_items(pool: &PgPool, ids: &[Uuid]) -> AppResult<u64> {
    if ids.is_empty() {
        return Ok(0);
    }
    let result = sqlx::query(
        "UPDATE food_items SET active = false, consumed_at = NOW() WHERE id = ANY($1) AND active = true"
    )
    .bind(ids)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn bulk_reactivate_food_items(pool: &PgPool, ids: &[Uuid]) -> AppResult<u64> {
    if ids.is_empty() {
        return Ok(0);
    }
    let result = sqlx::query(
        "UPDATE food_items SET active = true, consumed_at = NULL WHERE id = ANY($1) AND active = false"
    )
    .bind(ids)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn reactivate_food_item(pool: &PgPool, id: Uuid) -> AppResult<()> {
    let result = sqlx::query!(
        "UPDATE food_items SET active = true, consumed_at = NULL WHERE id = $1 AND active = false",
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound(format!(
            "Food item with id {} not found or already active",
            id
        )));
    }

    Ok(())
}
