use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::error::AppResult;
use crate::models::{CreateGroceryTrip, GroceryTrip, GroceryTripWithItems, ListParams, UpdateGroceryTrip};

pub async fn list_grocery_trips(
    State(pool): State<PgPool>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<GroceryTrip>>> {
    let trips = db::list_grocery_trips(&pool, &params).await?;
    Ok(Json(trips))
}

pub async fn get_grocery_trip(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<GroceryTripWithItems>> {
    let trip = db::get_grocery_trip(&pool, id).await?;
    Ok(Json(trip))
}

pub async fn create_grocery_trip(
    State(pool): State<PgPool>,
    Json(data): Json<CreateGroceryTrip>,
) -> AppResult<(StatusCode, Json<GroceryTrip>)> {
    let trip = db::create_grocery_trip(&pool, data).await?;
    Ok((StatusCode::CREATED, Json(trip)))
}

pub async fn update_grocery_trip(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateGroceryTrip>,
) -> AppResult<Json<GroceryTrip>> {
    let trip = db::update_grocery_trip(&pool, id, data).await?;
    Ok(Json(trip))
}

pub async fn delete_grocery_trip(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    db::delete_grocery_trip(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
