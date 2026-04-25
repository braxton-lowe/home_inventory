use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Meal {
    pub id: Uuid,
    pub name: String,
    pub made_on: NaiveDate,
    pub servings: i32,
    pub last_eaten: Option<NaiveDate>,
    pub rating: String,
    pub ingredients: Option<String>,
    pub recipe_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMeal {
    pub name: String,
    pub made_on: NaiveDate,
    pub servings: i32,
    pub last_eaten: Option<NaiveDate>,
    pub rating: String,
    pub ingredients: Option<String>,
    pub recipe_link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMeal {
    pub name: String,
    pub made_on: NaiveDate,
    pub servings: i32,
    pub last_eaten: Option<NaiveDate>,
    pub rating: String,
    pub ingredients: Option<String>,
    pub recipe_link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MealSuggestion {
    pub meal: Meal,
    pub matched_ingredients: Vec<String>,
    pub missing_ingredients: Vec<String>,
    pub match_percentage: f64,
}
