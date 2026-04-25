use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BulkItemIds {
    pub ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct BulkResult {
    pub affected: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FoodItem {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub item_type: String,
    pub brand: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub unit: String,
    pub price: Option<Decimal>,
    pub expiration_date: Option<NaiveDate>,
    pub purchase_date: NaiveDate,
    pub notes: Option<String>,
    pub location_id: Uuid,
    pub active: bool,
    pub consumed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFoodItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub brand: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub unit: String,
    pub price: Option<Decimal>,
    pub expiration_date: Option<NaiveDate>,
    pub purchase_date: NaiveDate,
    pub notes: Option<String>,
    pub location_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFoodItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub brand: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub unit: String,
    pub price: Option<Decimal>,
    pub expiration_date: Option<NaiveDate>,
    pub purchase_date: NaiveDate,
    pub notes: Option<String>,
    pub location_id: Uuid,
}
