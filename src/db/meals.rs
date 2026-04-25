use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{CreateMeal, ListParams, Meal, MealSuggestion, UpdateMeal};

pub async fn list_meals(pool: &PgPool, params: &ListParams) -> AppResult<Vec<Meal>> {
    let sort_col = params.sort_column(
        &["name", "made_on", "servings", "rating", "ingredients", "last_eaten"],
        "made_on",
    );
    let sort_dir = params.sort_direction_or("DESC");
    let limit = params.limit_or(50);
    let offset = params.offset_or(0);
    let search = params.search_filter();

    let where_clause = if search.is_some() {
        "WHERE name ILIKE $3 OR COALESCE(ingredients, '') ILIKE $3"
    } else {
        ""
    };

    let query = format!(
        "SELECT id, name, made_on, servings, last_eaten, rating, ingredients, recipe_link, created_at, updated_at FROM meals {} ORDER BY {} {} LIMIT $1 OFFSET $2",
        where_clause, sort_col, sort_dir
    );

    let mut q = sqlx::query_as::<_, Meal>(&query)
        .bind(limit)
        .bind(offset);
    if let Some(ref term) = search {
        q = q.bind(term);
    }
    let meals = q.fetch_all(pool).await?;

    Ok(meals)
}

pub async fn get_meal(pool: &PgPool, id: Uuid) -> AppResult<Meal> {
    let meal = sqlx::query_as!(
        Meal,
        "SELECT id, name, made_on, servings, last_eaten, rating, ingredients, recipe_link, created_at, updated_at FROM meals WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(meal)
}

pub async fn create_meal(pool: &PgPool, data: CreateMeal) -> AppResult<Meal> {
    let meal = sqlx::query_as!(
        Meal,
        r#"
        INSERT INTO meals (name, made_on, servings, last_eaten, rating, ingredients, recipe_link)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, made_on, servings, last_eaten, rating, ingredients, recipe_link, created_at, updated_at
        "#,
        data.name,
        data.made_on,
        data.servings,
        data.last_eaten,
        data.rating,
        data.ingredients,
        data.recipe_link
    )
    .fetch_one(pool)
    .await?;

    Ok(meal)
}

pub async fn update_meal(pool: &PgPool, id: Uuid, data: UpdateMeal) -> AppResult<Meal> {
    let meal = sqlx::query_as!(
        Meal,
        r#"
        UPDATE meals
        SET name = $1, made_on = $2, servings = $3, last_eaten = $4, rating = $5, ingredients = $6, recipe_link = $7, updated_at = NOW()
        WHERE id = $8
        RETURNING id, name, made_on, servings, last_eaten, rating, ingredients, recipe_link, created_at, updated_at
        "#,
        data.name,
        data.made_on,
        data.servings,
        data.last_eaten,
        data.rating,
        data.ingredients,
        data.recipe_link,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(meal)
}

pub async fn suggest_meals(pool: &PgPool) -> AppResult<Vec<MealSuggestion>> {
    // Get all active food item names (lowercased for matching)
    let active_items: Vec<String> = sqlx::query_scalar(
        "SELECT LOWER(name) FROM food_items WHERE active = true"
    )
    .fetch_all(pool)
    .await?;

    // Get all meals that have ingredients
    let meals = sqlx::query_as::<_, Meal>(
        "SELECT id, name, made_on, servings, last_eaten, rating, ingredients, recipe_link, created_at, updated_at FROM meals WHERE ingredients IS NOT NULL AND ingredients != '' ORDER BY rating DESC, last_eaten ASC NULLS FIRST"
    )
    .fetch_all(pool)
    .await?;

    // Words too short or generic to be meaningful matches
    let stop_words: std::collections::HashSet<&str> = [
        "a", "an", "the", "of", "or", "and", "to", "in", "for", "with",
        "lb", "lbs", "oz", "bag", "box", "jar", "can", "cup", "cups",
        "tsp", "tbsp", "bunch", "pkg", "pack", "piece", "pieces",
        "large", "small", "medium", "fresh", "frozen", "dried",
        "made", "ready", "etc",
    ].iter().copied().collect();

    // Build searchable tokens from each item name
    // e.g. "baby spinach" -> ["baby spinach", "spinach"]
    // e.g. "garbanzo beans/chickpeas" -> ["garbanzo beans/chickpeas", "garbanzo", "beans", "chickpeas"]
    let item_tokens: Vec<Vec<String>> = active_items.iter().map(|name| {
        let mut tokens = vec![name.clone()];
        for word in name.split(|c: char| c.is_whitespace() || c == '/' || c == '-') {
            let w = word.trim().to_lowercase();
            if w.len() >= 3 && !stop_words.contains(w.as_str()) {
                tokens.push(w);
            }
        }
        tokens
    }).collect();

    let mut suggestions: Vec<MealSuggestion> = Vec::new();

    for meal in meals {
        let ingredients_str = match &meal.ingredients {
            Some(s) => s.clone(),
            None => continue,
        };

        let parsed: Vec<String> = ingredients_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if parsed.is_empty() {
            continue;
        }

        let mut matched = Vec::new();
        let mut missing = Vec::new();

        for ingredient in &parsed {
            let ingredient_lower = ingredient.to_lowercase();

            // Check if any token from any active item appears in this ingredient string
            let is_match = item_tokens.iter().any(|tokens| {
                tokens.iter().any(|token| {
                    ingredient_lower.contains(token)
                })
            });

            if is_match {
                matched.push(ingredient.clone());
            } else {
                missing.push(ingredient.clone());
            }
        }

        let pct = (matched.len() as f64 / parsed.len() as f64) * 100.0;

        suggestions.push(MealSuggestion {
            meal,
            matched_ingredients: matched,
            missing_ingredients: missing,
            match_percentage: (pct * 10.0).round() / 10.0,
        });
    }

    // Sort by match percentage descending
    suggestions.sort_by(|a, b| {
        b.match_percentage
            .partial_cmp(&a.match_percentage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Only return meals that have at least one match
    suggestions.retain(|s| s.match_percentage > 0.0);

    Ok(suggestions)
}

pub async fn delete_meal(pool: &PgPool, id: Uuid) -> AppResult<()> {
    let result = sqlx::query!("DELETE FROM meals WHERE id = $1", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound(format!(
            "Meal with id {} not found",
            id
        )));
    }

    Ok(())
}
