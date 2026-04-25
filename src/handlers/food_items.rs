use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use std::collections::HashMap;

use crate::db;
use crate::error::AppResult;
use crate::models::{BulkItemIds, BulkResult, CreateFoodItem, FoodItem, ListParams, UpdateFoodItem};

pub async fn list_food_items(
    State(pool): State<PgPool>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<FoodItem>>> {
    let items = db::list_food_items(&pool, &params).await?;
    Ok(Json(items))
}

pub async fn get_food_item(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<FoodItem>> {
    let item = db::get_food_item(&pool, id).await?;
    Ok(Json(item))
}

pub async fn create_food_item(
    State(pool): State<PgPool>,
    Json(data): Json<CreateFoodItem>,
) -> AppResult<(StatusCode, Json<FoodItem>)> {
    let item = db::create_food_item(&pool, data).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update_food_item(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateFoodItem>,
) -> AppResult<Json<FoodItem>> {
    let item = db::update_food_item(&pool, id, data).await?;
    Ok(Json(item))
}

pub async fn autocomplete_food_items(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> AppResult<Json<Vec<FoodItem>>> {
    let query = params.get("q").map(|s| s.as_str()).unwrap_or("");
    if query.is_empty() {
        return Ok(Json(vec![]));
    }
    let items = db::autocomplete_food_items(&pool, query).await?;
    Ok(Json(items))
}

pub async fn delete_food_item(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    db::deactivate_food_item(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn bulk_deactivate_food_items(
    State(pool): State<PgPool>,
    Json(data): Json<BulkItemIds>,
) -> AppResult<Json<BulkResult>> {
    let affected = db::bulk_deactivate_food_items(&pool, &data.ids).await?;
    Ok(Json(BulkResult { affected }))
}

pub async fn bulk_reactivate_food_items(
    State(pool): State<PgPool>,
    Json(data): Json<BulkItemIds>,
) -> AppResult<Json<BulkResult>> {
    let affected = db::bulk_reactivate_food_items(&pool, &data.ids).await?;
    Ok(Json(BulkResult { affected }))
}

pub async fn reactivate_food_item(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    db::reactivate_food_item(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
