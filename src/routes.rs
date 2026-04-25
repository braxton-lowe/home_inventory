use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::auth::{basic_auth_middleware, AuthConfig};
use crate::handlers;

pub fn create_routes(pool: PgPool, auth_config: AuthConfig) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard))
        .route("/health", get(handlers::health_check))
        .nest("/api/v1", api_routes(pool))
        .layer(middleware::from_fn_with_state(
            auth_config,
            basic_auth_middleware,
        ))
}

fn api_routes(pool: PgPool) -> Router {
    Router::new()
        // Location routes
        .route("/locations", get(handlers::list_locations))
        .route("/locations", post(handlers::create_location))
        .route("/locations/:id", get(handlers::get_location))
        .route("/locations/:id", put(handlers::update_location))
        .route("/locations/:id", delete(handlers::delete_location))
        // Food item routes
        .route("/items", get(handlers::list_food_items))
        .route("/items", post(handlers::create_food_item))
        .route("/items/autocomplete", get(handlers::autocomplete_food_items))
        .route("/items/:id", get(handlers::get_food_item))
        .route("/items/:id", put(handlers::update_food_item))
        .route("/items/:id", delete(handlers::delete_food_item))
        .route("/items/:id/reactivate", put(handlers::reactivate_food_item))
        .route("/items/bulk/deactivate", post(handlers::bulk_deactivate_food_items))
        .route("/items/bulk/reactivate", post(handlers::bulk_reactivate_food_items))
        // Grocery trip routes
        .route("/trips", get(handlers::list_grocery_trips))
        .route("/trips", post(handlers::create_grocery_trip))
        .route("/trips/:id", get(handlers::get_grocery_trip))
        .route("/trips/:id", put(handlers::update_grocery_trip))
        .route("/trips/:id", delete(handlers::delete_grocery_trip))
        // Meal routes
        .route("/meals", get(handlers::list_meals))
        .route("/meals", post(handlers::create_meal))
        .route("/meals/suggestions", get(handlers::suggest_meals))
        .route("/meals/:id", get(handlers::get_meal))
        .route("/meals/:id", put(handlers::update_meal))
        .route("/meals/:id", delete(handlers::delete_meal))
        // Import routes
        .route("/import/items", post(handlers::import_items))
        .route("/import/trips", post(handlers::import_trips))
        .route("/import/meals", post(handlers::import_meals))
        .with_state(pool)
}
