use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::error::AppResult;
use crate::models::{CreateMeal, ListParams, Meal, MealSuggestion, UpdateMeal};

pub async fn list_meals(
    State(pool): State<PgPool>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<Meal>>> {
    let meals = db::list_meals(&pool, &params).await?;
    Ok(Json(meals))
}

pub async fn get_meal(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Meal>> {
    let meal = db::get_meal(&pool, id).await?;
    Ok(Json(meal))
}

pub async fn create_meal(
    State(pool): State<PgPool>,
    Json(data): Json<CreateMeal>,
) -> AppResult<(StatusCode, Json<Meal>)> {
    let meal = db::create_meal(&pool, data).await?;
    Ok((StatusCode::CREATED, Json(meal)))
}

pub async fn update_meal(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateMeal>,
) -> AppResult<Json<Meal>> {
    let meal = db::update_meal(&pool, id, data).await?;
    Ok(Json(meal))
}

pub async fn suggest_meals(
    State(pool): State<PgPool>,
) -> AppResult<Json<Vec<MealSuggestion>>> {
    let suggestions = db::suggest_meals(&pool).await?;
    Ok(Json(suggestions))
}

pub async fn delete_meal(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    db::delete_meal(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
