use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::FoodItem;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroceryTrip {
    pub id: Uuid,
    pub trip_date: NaiveDate,
    pub store_name: String,
    pub total_spent: Option<Decimal>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct GroceryTripWithItems {
    #[serde(flatten)]
    pub trip: GroceryTrip,
    pub items: Vec<FoodItem>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroceryTrip {
    pub trip_date: NaiveDate,
    pub store_name: String,
    pub total_spent: Option<Decimal>,
    pub notes: Option<String>,
    pub item_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroceryTrip {
    pub trip_date: NaiveDate,
    pub store_name: String,
    pub total_spent: Option<Decimal>,
    pub notes: Option<String>,
}
