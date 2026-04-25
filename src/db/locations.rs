use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{CreateLocation, ListParams, Location, UpdateLocation};

pub async fn list_locations(pool: &PgPool, params: &ListParams) -> AppResult<Vec<Location>> {
    let sort_col = params.sort_column(&["name", "description", "created_at"], "name");
    let sort_dir = params.sort_direction_or("ASC");
    let limit = params.limit_or(50);
    let offset = params.offset_or(0);
    let search = params.search_filter();

    let where_clause = if search.is_some() {
        "WHERE name ILIKE $3 OR COALESCE(description, '') ILIKE $3"
    } else {
        ""
    };

    let query = format!(
        "SELECT id, name, description, created_at, updated_at FROM locations {} ORDER BY {} {} LIMIT $1 OFFSET $2",
        where_clause, sort_col, sort_dir
    );

    let mut q = sqlx::query_as::<_, Location>(&query)
        .bind(limit)
        .bind(offset);
    if let Some(ref term) = search {
        q = q.bind(term);
    }
    let locations = q.fetch_all(pool).await?;

    Ok(locations)
}

pub async fn get_location(pool: &PgPool, id: Uuid) -> AppResult<Location> {
    let location = sqlx::query_as!(
        Location,
        "SELECT id, name, description, created_at, updated_at FROM locations WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(location)
}

pub async fn create_location(pool: &PgPool, data: CreateLocation) -> AppResult<Location> {
    let location = sqlx::query_as!(
        Location,
        r#"
        INSERT INTO locations (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at, updated_at
        "#,
        data.name,
        data.description
    )
    .fetch_one(pool)
    .await?;

    Ok(location)
}

pub async fn update_location(
    pool: &PgPool,
    id: Uuid,
    data: UpdateLocation,
) -> AppResult<Location> {
    let location = sqlx::query_as!(
        Location,
        r#"
        UPDATE locations
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description, created_at, updated_at
        "#,
        data.name,
        data.description,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(location)
}

pub async fn delete_location(pool: &PgPool, id: Uuid) -> AppResult<()> {
    let result = sqlx::query!("DELETE FROM locations WHERE id = $1", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound(format!(
            "Location with id {} not found",
            id
        )));
    }

    Ok(())
}
