use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TripItem {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub food_item_id: Uuid,
}
