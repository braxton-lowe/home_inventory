use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::error::AppResult;
use crate::models::{CreateLocation, ListParams, Location, UpdateLocation};

pub async fn list_locations(
    State(pool): State<PgPool>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<Location>>> {
    let locations = db::list_locations(&pool, &params).await?;
    Ok(Json(locations))
}

pub async fn get_location(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Location>> {
    let location = db::get_location(&pool, id).await?;
    Ok(Json(location))
}

pub async fn create_location(
    State(pool): State<PgPool>,
    Json(data): Json<CreateLocation>,
) -> AppResult<(StatusCode, Json<Location>)> {
    let location = db::create_location(&pool, data).await?;
    Ok((StatusCode::CREATED, Json(location)))
}

pub async fn update_location(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdateLocation>,
) -> AppResult<Json<Location>> {
    let location = db::update_location(&pool, id, data).await?;
    Ok(Json(location))
}

pub async fn delete_location(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    db::delete_location(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
