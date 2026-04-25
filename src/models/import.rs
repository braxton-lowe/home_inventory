use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

/// Parses a date string trying multiple formats: YYYY-MM-DD, M/D/YYYY, M-D-YYYY.
fn parse_flexible_date(s: &str) -> Result<NaiveDate, String> {
    let s = s.trim();
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(s, "%-m/%-d/%Y"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%-m-%-d-%Y"))
        .map_err(|e| format!("invalid date '{}': {}", s, e))
}

/// Deserializes a NaiveDate from flexible formats.
fn deserialize_flexible_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    let s = String::deserialize(deserializer)?;
    parse_flexible_date(&s).map_err(serde::de::Error::custom)
}

/// Deserializes an Option<NaiveDate> from flexible formats, treating empty/invalid as None.
fn deserialize_flexible_date_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<NaiveDate>, D::Error> {
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => parse_flexible_date(&s).map(Some).map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Deserialize)]
pub struct ItemCsvRow {
    pub location: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub brand: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub unit: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub price: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_flexible_date_opt")]
    pub expiration_date: Option<NaiveDate>,
    #[serde(deserialize_with = "deserialize_flexible_date_opt")]
    pub purchase_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TripCsvRow {
    #[serde(deserialize_with = "deserialize_flexible_date")]
    pub trip_date: NaiveDate,
    pub store_name: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub total_spent: Option<Decimal>,
    pub trip_notes: Option<String>,
    pub location: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub brand: Option<String>,
    pub name: String,
    pub quantity: Decimal,
    pub unit: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub price: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_flexible_date_opt")]
    pub expiration_date: Option<NaiveDate>,
    #[serde(deserialize_with = "deserialize_flexible_date_opt")]
    pub purchase_date: Option<NaiveDate>,
    pub item_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MealCsvRow {
    pub name: String,
    #[serde(deserialize_with = "deserialize_flexible_date")]
    pub made_on: NaiveDate,
    pub servings: i32,
    #[serde(deserialize_with = "deserialize_flexible_date_opt")]
    pub last_eaten: Option<NaiveDate>,
    pub rating: String,
    pub ingredients: Option<String>,
    pub recipe_link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub rows_processed: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations_created: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food_items_created: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food_items_updated: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trips_created: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trips_updated: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trip_items_linked: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meals_created: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meals_updated: Option<usize>,
}
