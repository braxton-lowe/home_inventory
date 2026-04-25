use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;

pub async fn link_items_to_trip(
    pool: &PgPool,
    trip_id: Uuid,
    item_ids: &[Uuid],
) -> AppResult<()> {
    for item_id in item_ids {
        sqlx::query!(
            r#"
            INSERT INTO trip_items (trip_id, food_item_id)
            VALUES ($1, $2)
            ON CONFLICT (trip_id, food_item_id) DO NOTHING
            "#,
            trip_id,
            item_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn unlink_all_items_from_trip(pool: &PgPool, trip_id: Uuid) -> AppResult<()> {
    sqlx::query!("DELETE FROM trip_items WHERE trip_id = $1", trip_id)
        .execute(pool)
        .await?;

    Ok(())
}
