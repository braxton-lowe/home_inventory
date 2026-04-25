use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::db;
use crate::error::{AppError, AppResult};
use crate::models::{ImportResult, ItemCsvRow, MealCsvRow, TripCsvRow};

async fn extract_csv_from_multipart(mut multipart: Multipart) -> AppResult<Vec<u8>> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::ValidationError(format!("Failed to read multipart field: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let bytes = field
                .bytes()
                .await
                .map_err(|e| AppError::ValidationError(format!("Failed to read file data: {}", e)))?;
            return Ok(bytes.to_vec());
        }
    }

    Err(AppError::ValidationError(
        "No 'file' field found in multipart upload".to_string(),
    ))
}

fn parse_csv<T: serde::de::DeserializeOwned>(data: &[u8]) -> AppResult<Vec<T>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(data);

    let mut rows = Vec::new();
    for (i, result) in reader.deserialize().enumerate() {
        let row: T = result.map_err(|e| {
            AppError::ValidationError(format!("Row {}: {}", i + 2, e))
        })?;
        rows.push(row);
    }

    if rows.is_empty() {
        return Err(AppError::ValidationError("CSV file contains no data rows".to_string()));
    }

    Ok(rows)
}

pub async fn import_items(
    State(pool): State<PgPool>,
    multipart: Multipart,
) -> AppResult<(StatusCode, Json<ImportResult>)> {
    let csv_data = extract_csv_from_multipart(multipart).await?;
    let rows: Vec<ItemCsvRow> = parse_csv(&csv_data)?;
    let result = db::process_items_import(&pool, rows).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn import_trips(
    State(pool): State<PgPool>,
    multipart: Multipart,
) -> AppResult<(StatusCode, Json<ImportResult>)> {
    let csv_data = extract_csv_from_multipart(multipart).await?;
    let rows: Vec<TripCsvRow> = parse_csv(&csv_data)?;
    let result = db::process_trip_import(&pool, rows).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn import_meals(
    State(pool): State<PgPool>,
    multipart: Multipart,
) -> AppResult<(StatusCode, Json<ImportResult>)> {
    let csv_data = extract_csv_from_multipart(multipart).await?;
    let rows: Vec<MealCsvRow> = parse_csv(&csv_data)?;
    let result = db::process_meals_import(&pool, rows).await?;
    Ok((StatusCode::CREATED, Json(result)))
}
