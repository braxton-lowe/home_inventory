# How the Rust REST API Works - A Python Developer's Guide

This document walks through how the entire application flows from startup to handling a single API request. If you're familiar with Flask or FastAPI, many concepts will feel familiar, but with Rust's type safety and async model.

---

## Quick Comparison to Python

| Python (FastAPI) | Rust (Axum) | Purpose |
|-----------------|-------------|---------|
| `@app.get("/items")` | `Router::new().route("/items", get(handler))` | Define route |
| `async def handler()` | `async fn handler()` | Async function |
| Pydantic models | `struct` with `#[derive(Serialize)]` | Data validation |
| SQLAlchemy | SQLx | Database queries |
| `@app.on_event("startup")` | Code in `main()` before `serve()` | Startup logic |
| Connection pool injection | `State<PgPool>` | Shared state |

---

## Application Startup Flow

### 1. **`main.rs`** - Entry Point (Like `app.py` or `main.py`)

```
User runs: cargo run
         ↓
    main.rs starts
```

**What happens:**

```rust
#[tokio::main]  // ← Sets up async runtime (like asyncio in Python)
async fn main() -> anyhow::Result<()> {
    // Step 1: Initialize logging with environment filter
    // (like Python's logging.basicConfig with level config)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,home_food_inventory=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Step 2: Load config (reads DATABASE_URL, HOST, PORT from env)
    // Config::from_env() calls dotenvy::dotenv() internally
    let config = config::Config::from_env()?;

    // Step 3: Create database connection pool
    // Like: pool = await create_pool(DATABASE_URL)
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Step 4: Run database migrations (ensures tables exist)
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Step 5: Build the application router
    let app = routes::create_routes(pool);

    // Step 6: Start the server
    // Like: uvicorn.run(app, host="0.0.0.0", port=3000)
    let addr = config.addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

**Key differences from Python:**
- `#[tokio::main]` - Rust requires an async runtime (Tokio). Python's `asyncio` is built-in.
- Everything returns `Result<T, E>` - Rust forces you to handle errors explicitly
- `?` operator - Propagates errors up (like `raise` but automatic)

---

### 2. **`config.rs`** - Configuration (Like Python's `os.getenv()`)

**Purpose:** Centralize environment variable loading

```rust
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();  // ← Load .env file here

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://grocery_user:dev_password@localhost:5432/home_food_inventory".to_string());

        let host = env::var("HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;

        Ok(Config { database_url, host, port })
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
```

**Python equivalent:**
```python
class Config:
    database_url = os.getenv("DATABASE_URL")
    host = os.getenv("HOST", "0.0.0.0")
    port = int(os.getenv("PORT", "3000"))
```

---

### 3. **SQLx CLI and the `.env` File** - Database Migrations

Before your application runs, you need to set up the database schema. This is done using the **SQLx CLI** tool and migration files.

**How SQLx CLI reads configuration:**

```bash
# When you run this command:
sqlx migrate run

# SQLx CLI does the following:
# 1. Looks for a .env file in the current directory
# 2. Reads DATABASE_URL from it
# 3. Connects to that PostgreSQL database
# 4. Runs all .sql files in migrations/ directory in order
```

**Your `.env` file:**
```env
DATABASE_URL=postgresql://grocery_user:dev_password@localhost:5432/home_food_inventory
HOST=0.0.0.0
PORT=3000
RUST_LOG=info,home_food_inventory=debug
```

**Python equivalent:**
In Python, you might use Alembic for SQLAlchemy migrations:
```bash
# alembic.ini contains database URL
alembic upgrade head
```

**Key points:**
- SQLx CLI is a **separate tool** from the SQLx library used in your app
- It reads `.env` automatically (uses the `dotenvy` crate under the hood)
- You can override with environment variable: `DATABASE_URL=... sqlx migrate run`
- Migration files are timestamped (e.g., `20240101000001_create_locations.sql`)
- Migrations run in order and are tracked in a `_sqlx_migrations` table

**What happens during migration:**

```
sqlx migrate run
      ↓
1. Read DATABASE_URL from .env
      ↓
2. Connect to PostgreSQL
      ↓
3. Check _sqlx_migrations table (creates if missing)
      ↓
4. For each .sql file not yet applied:
   - Run the SQL commands
   - Record migration in _sqlx_migrations table
      ↓
5. Done! Tables are now created
```

**Inside your app (main.rs):**
After SQLx CLI creates the tables, your application also runs migrations at startup:
```rust
sqlx::migrate!("./migrations").run(&pool).await?;
```

This is a **compile-time macro** that:
- Embeds all migration files into your binary
- Automatically runs any pending migrations when the app starts
- Ensures the database is always up-to-date

So migrations can be run two ways:
1. **Manually:** `sqlx migrate run` (before development/deployment)
2. **Automatically:** On app startup via `sqlx::migrate!()` macro

---

### 4. **`routes.rs`** - Define API Routes (Like Flask's `@app.route` or FastAPI's `@app.get`)

**Purpose:** Map URL paths to handler functions

```rust
pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .nest("/api/v1", api_routes(pool))  // ← Nested under /api/v1 prefix
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
        .route("/items/:id", get(handlers::get_food_item))
        .route("/items/:id", put(handlers::update_food_item))
        .route("/items/:id", delete(handlers::delete_food_item))
        // Grocery trip routes
        .route("/trips", get(handlers::list_grocery_trips))
        .route("/trips", post(handlers::create_grocery_trip))
        .route("/trips/:id", get(handlers::get_grocery_trip))
        .route("/trips/:id", put(handlers::update_grocery_trip))
        .route("/trips/:id", delete(handlers::delete_grocery_trip))
        // Meal routes
        .route("/meals", get(handlers::list_meals))
        .route("/meals", post(handlers::create_meal))
        .route("/meals/:id", get(handlers::get_meal))
        .route("/meals/:id", put(handlers::update_meal))
        .route("/meals/:id", delete(handlers::delete_meal))
        // Import routes (upsert — re-importing updates existing records)
        .route("/import/items", post(handlers::import_items))
        .route("/import/trips", post(handlers::import_trips))
        .route("/import/meals", post(handlers::import_meals))
        // Inject database pool as shared state for all handlers
        .with_state(pool)
}
```

**Python (FastAPI) equivalent:**
```python
from fastapi import FastAPI, Depends
from sqlalchemy.ext.asyncio import AsyncSession

app = FastAPI()

@app.get("/api/v1/items")
async def list_items(db: AsyncSession = Depends(get_db)):
    # handler code
    pass
```

**Key concept:** `.with_state(pool)` makes the database connection pool available to all handlers via `State<PgPool>` (like FastAPI's dependency injection).

---

## Request Handling Flow - Example: `GET /api/v1/items`

Let's trace what happens when a client makes this request:

```bash
curl http://localhost:3000/api/v1/items
```

### Step-by-Step Execution:

```
1. Request arrives at server
         ↓
2. Axum router matches route → "/api/v1/items" with GET method
         ↓
3. Router calls: handlers/food_items.rs::list_food_items()
         ↓
4. Handler extracts database pool from shared state
         ↓
5. Handler calls: db/food_items.rs::list_food_items()
         ↓
6. Database function runs SQL query via SQLx
         ↓
7. SQLx returns Vec<FoodItem> (vector of food items)
         ↓
8. Handler wraps result in Json() and returns
         ↓
9. Axum serializes to JSON and sends HTTP response
```

---

### File Details for `GET /api/v1/items`:

#### **`handlers/food_items.rs`** - HTTP Handler (Like Flask view or FastAPI path operation)

```rust
use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::db;
use crate::error::AppResult;
use crate::models::FoodItem;

// Handler function - receives HTTP request, returns HTTP response
pub async fn list_food_items(
    State(pool): State<PgPool>,  // ← Database pool extracted from shared state
) -> AppResult<Json<Vec<FoodItem>>> {
    // Call database layer
    let items = db::list_food_items(&pool).await?;

    // Return JSON response
    Ok(Json(items))
}
```

**Python (FastAPI) equivalent:**
```python
from fastapi import Depends
from sqlalchemy.ext.asyncio import AsyncSession

@app.get("/api/v1/items", response_model=list[FoodItem])
async def list_items(db: AsyncSession = Depends(get_db)):
    items = await get_all_items(db)
    return items  # FastAPI auto-serializes to JSON
```

**Key points:**
- `State(PgPool)` - Extracts the connection pool from shared state (like FastAPI's `Depends`)
- `Json<Vec<FoodItem>>` - Tells Axum to serialize the vector to JSON
- `AppResult<T>` - Type alias for `Result<T, AppError>`, handler errors get converted to HTTP error responses
- `async fn` - Handler runs asynchronously (like Python's `async def`)

---

#### **`db/food_items.rs`** - Database Layer (Like SQLAlchemy queries)

```rust
use sqlx::PgPool;
use crate::error::AppResult;
use crate::models::FoodItem;

// Pure database logic - no HTTP concepts here
pub async fn list_food_items(pool: &PgPool) -> AppResult<Vec<FoodItem>> {
    // sqlx::query_as! is compile-time checked - it validates SQL at build time!
    let items = sqlx::query_as!(
        FoodItem,
        r#"
        SELECT
            id,
            type as "item_type!",
            brand, name, quantity, unit, price,
            expiration_date, purchase_date, notes, location_id,
            created_at, updated_at
        FROM food_items
        ORDER BY purchase_date DESC, name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(items)
}
```

**Python (SQLAlchemy) equivalent:**
```python
from sqlalchemy import select
from sqlalchemy.ext.asyncio import AsyncSession

async def list_food_items(db: AsyncSession) -> list[FoodItem]:
    result = await db.execute(
        select(FoodItem).order_by(FoodItem.purchase_date.desc(), FoodItem.name)
    )
    return result.scalars().all()
```

**Key differences:**
- `query_as!` - Compile-time SQL checking! If your SQL is wrong or column types don't match the struct, the code won't compile
- `type as "item_type!"` - SQL column alias to map the `type` column (a reserved word) to the struct field `item_type`. The `!` tells SQLx the column is NOT NULL
- `&PgPool` - Borrowing the pool (no ownership transfer)
- `.await?` - Await the async operation, propagate errors up

---

#### **`models/food_item.rs`** - Data Model (Like Pydantic or SQLAlchemy model)

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FoodItem {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub item_type: String,           // ← Field is "item_type", serializes as "type" in JSON
    pub brand: Option<String>,
    pub name: String,
    pub quantity: Decimal,           // ← Decimal for precise quantities
    pub unit: String,
    pub price: Option<Decimal>,      // ← Decimal for precise prices
    pub expiration_date: Option<NaiveDate>,
    pub purchase_date: NaiveDate,
    pub notes: Option<String>,
    pub location_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Struct for creating new items (no id, timestamps)
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
```

**Python (Pydantic) equivalent:**
```python
from pydantic import BaseModel, Field
from datetime import datetime, date
from uuid import UUID
from typing import Optional
from decimal import Decimal

class FoodItem(BaseModel):
    id: UUID
    type: str = Field(alias="type")
    brand: Optional[str]
    name: str
    quantity: Decimal
    unit: str
    price: Optional[Decimal]
    expiration_date: Optional[date]
    purchase_date: date
    notes: Optional[str]
    location_id: UUID
    created_at: datetime
    updated_at: datetime

class CreateFoodItem(BaseModel):
    type: str
    brand: Optional[str]
    # ... rest of fields
```

**Key points:**
- `#[derive(Serialize, Deserialize)]` - Enables JSON conversion (like Pydantic)
- `#[derive(FromRow)]` - Enables loading from database rows
- `Option<T>` - Rust's way of saying "nullable" (like Python's `Optional[T]`)
- Two structs: `FoodItem` (full) and `CreateFoodItem` (for POST requests)

---

## Complete Request Flow Diagram

```
HTTP Request: GET /api/v1/items
         |
         v
┌────────────────────────┐
│   Axum Server          │  (routes.rs)
│   "I got a GET to      │
│    /api/v1/items"      │
└────────┬───────────────┘
         |
         v
┌────────────────────────┐
│   Handler Layer        │  (handlers/food_items.rs)
│   list_items()         │
│   - Extract DB pool    │
│   - Call DB function   │
│   - Return JSON        │
└────────┬───────────────┘
         |
         v
┌────────────────────────┐
│   Database Layer       │  (db/food_items.rs)
│   get_all_items()      │
│   - Execute SQL query  │
│   - Map rows to structs│
└────────┬───────────────┘
         |
         v
┌────────────────────────┐
│   PostgreSQL Database  │
│   SELECT * FROM        │
│   food_items...        │
└────────┬───────────────┘
         |
         v
    Vec<FoodItem> ← Returns vector of structs
         |
         v
    Serialize to JSON
         |
         v
HTTP Response: 200 OK
{
  "items": [...]
}
```

---

## Error Handling Flow

If something goes wrong at any step:

```rust
// In db/food_items.rs
pub async fn list_food_items(pool: &PgPool) -> AppResult<Vec<FoodItem>> {
    let items = sqlx::query_as!(...)
        .fetch_all(pool)
        .await?;  // ← If this fails, error propagates up via From<sqlx::Error>
    Ok(items)
}

// In handlers/food_items.rs
pub async fn list_food_items(
    State(pool): State<PgPool>,
) -> AppResult<Json<Vec<FoodItem>>> {
    let items = db::list_food_items(&pool).await?;  // ← Error caught here
    Ok(Json(items))
}

// AppError implementation converts to HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(ref e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::ValidationError(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalError(ref msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str())
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
```

The `?` operator automatically converts errors and propagates them up the call stack until they hit the handler, which converts them to HTTP responses.

---

## CSV Import Flow - Bulk Data Loading

The API supports bulk importing data via CSV file uploads. There are two import endpoints:

- `POST /api/v1/import/items` - Import food items (no trip)
- `POST /api/v1/import/trips` - Import a grocery trip with items

### How It Works

```
1. Client sends multipart/form-data with a "file" field containing CSV
         ↓
2. Handler extracts CSV bytes from multipart upload
         ↓
3. CSV is parsed into typed rows (ItemCsvRow or TripCsvRow)
   - Uses the `csv` crate with serde Deserialize
   - Bad rows fail the entire import with row number in error message
         ↓
4. Database processing runs in a single transaction:
   a. Resolve locations by name (create new ones if needed)
   b. Create food items
   c. (Trip import only) Group rows by (trip_date, store_name),
      create trips, and link items to trips
         ↓
5. Return ImportResult with counts of everything created
```

### Items CSV Format

```csv
location,type,brand,name,quantity,unit,price,expiration_date,purchase_date,notes
pantry,produce,,Organic Bananas,6,count,3.99,,2024-02-07,
fridge,dairy,Horizon,Whole Milk,1,gallon,5.99,2024-02-20,2024-02-07,
```

### Trip CSV Format

```csv
trip_date,store_name,total_spent,trip_notes,location,type,brand,name,quantity,unit,price,expiration_date,purchase_date,item_notes
2024-02-07,Whole Foods,45.67,,pantry,produce,,Organic Bananas,6,count,3.99,,2024-02-07,
2024-02-07,Whole Foods,45.67,,fridge,dairy,Horizon,Whole Milk,1,gallon,5.99,2024-02-20,2024-02-07,
```

Rows with the same `trip_date` and `store_name` are grouped into a single trip. The `total_spent` and `trip_notes` are taken from the first row of each group.

### Key Design Decisions

- **Fail-fast**: One bad row rolls back the entire import. Error messages include the row number.
- **Locations resolved by name**: No UUIDs in CSVs. "pantry" maps to an existing location or creates a new one (case-insensitive match).
- **Transactional**: The entire import runs in one database transaction - either everything succeeds or nothing changes.

---

## Why This Architecture?

### **Separation of Concerns:**
1. **Handlers** - Know about HTTP (requests, responses, status codes)
2. **Database layer** - Pure SQL logic, no HTTP concepts
3. **Models** - Just data structures

**Benefits:**
- Easy to test database functions without HTTP
- Can swap out Axum for another framework without changing DB code
- Clear responsibilities

### **Type Safety:**
- If your SQL query returns wrong types, **code won't compile**
- If you forget to handle an error, **code won't compile**
- If you try to access a nullable field without checking, **code won't compile**

### **Async All The Way:**
- Just like Python's `async/await`
- Database queries don't block the server
- Can handle many concurrent requests efficiently

---

## Summary: Files and Their Roles

| File | Role | Python Equivalent |
|------|------|------------------|
| `main.rs` | Entry point, setup, start server | `main.py`, `app.py` |
| `config.rs` | Load environment variables | `config.py`, `settings.py` |
| `routes.rs` | Map URLs to handlers | `@app.route()`, `app.include_router()` |
| `handlers/*.rs` | HTTP request/response logic | FastAPI path operations, Flask views |
| `db/*.rs` | Database queries | SQLAlchemy queries, raw SQL functions |
| `models/*.rs` | Data structures | Pydantic models, dataclasses |
| `error.rs` | Error type definitions | Custom exception classes |

---

## Next Steps to Understanding

1. **Start the server:** `cargo run`
2. **Make a request:** `curl http://localhost:3000/api/v1/locations`
3. **Watch the logs:** See which functions are called
4. **Read the code top-to-bottom:** Start at `main.rs`, follow the flow
5. **Modify a handler:** Change the response, see what happens

The beauty of Rust is that if it compiles, it's very likely to work correctly! 🦀
