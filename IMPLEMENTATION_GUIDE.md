# Home Inventory - Implementation Guide

## Overview

This is a REST API for tracking grocery inventory built with Rust, Axum, SQLx, and PostgreSQL.

## Database Schema

### Tables

#### 1. locations
```sql
CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### 2. grocery_trips
```sql
CREATE TABLE grocery_trips (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trip_date DATE NOT NULL,
    store_name TEXT NOT NULL,
    total_spent DECIMAL(10,2) NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### 3. food_items
```sql
CREATE TABLE food_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type TEXT NOT NULL,
    brand TEXT,
    name TEXT NOT NULL,
    quantity DECIMAL(10,2) NOT NULL,
    unit TEXT NOT NULL,
    price DECIMAL(10,2),
    expiration_date DATE,
    purchase_date DATE NOT NULL,
    notes TEXT,
    location_id UUID NOT NULL REFERENCES locations(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### 4. trip_items (Junction Table)
```sql
CREATE TABLE trip_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trip_id UUID NOT NULL REFERENCES grocery_trips(id) ON DELETE CASCADE,
    food_item_id UUID NOT NULL REFERENCES food_items(id) ON DELETE CASCADE,
    UNIQUE(trip_id, food_item_id)
);
```

## API Endpoints

### Base URL: `http://localhost:3000/api/v1`

#### Dashboard
- `GET /` - HTML dashboard with tables, search, and pagination for all data

#### Health Check
- `GET /health` - Returns `{"status": "ok"}`

#### Locations
- `GET /locations` - List all locations
- `POST /locations` - Create a new location
- `GET /locations/:id` - Get location by ID
- `PUT /locations/:id` - Update location
- `DELETE /locations/:id` - Delete location

#### Food Items
- `GET /items` - List all food items
- `POST /items` - Create a new food item
- `GET /items/:id` - Get food item by ID
- `PUT /items/:id` - Update food item
- `DELETE /items/:id` - Delete food item

#### Grocery Trips
- `GET /trips` - List all grocery trips
- `POST /trips` - Create a new trip (with optional item_ids)
- `GET /trips/:id` - Get trip by ID (includes items)
- `PUT /trips/:id` - Update trip
- `DELETE /trips/:id` - Delete trip

#### Meals
- `GET /meals` - List all meals
- `POST /meals` - Create a new meal
- `GET /meals/:id` - Get meal by ID
- `PUT /meals/:id` - Update meal
- `DELETE /meals/:id` - Delete meal

#### Pagination & Sorting

All list endpoints (`GET /locations`, `/items`, `/trips`, `/meals`) support these query parameters:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit`   | int  | 50      | Max rows to return (1-100) |
| `offset`  | int  | 0       | Number of rows to skip |
| `sort`    | string | varies | Sort direction: `asc` or `desc` |
| `search`  | string | —      | Filter results with case-insensitive partial match |

Search fields per endpoint:
- **Items**: `name`, `brand`, `notes`
- **Locations**: `name`, `description`
- **Grocery Trips**: `store_name`, `notes`
- **Meals**: `name`, `ingredients`

Default sort directions:
- **Locations**: `name ASC`
- **Food Items**: `purchase_date DESC, name`
- **Grocery Trips**: `trip_date DESC`
- **Meals**: `made_on DESC, name`

**Examples:**
```bash
# First 10 items
curl "http://localhost:3000/api/v1/items?limit=10"

# Next 10 items (page 2)
curl "http://localhost:3000/api/v1/items?limit=10&offset=10"

# Oldest items first
curl "http://localhost:3000/api/v1/items?sort=asc"

# 20 most recent meals
curl "http://localhost:3000/api/v1/meals?limit=20&sort=desc"

# Search for items matching "chicken"
curl "http://localhost:3000/api/v1/items?search=chicken"

# Search trips by store name
curl "http://localhost:3000/api/v1/trips?search=costco"
```

#### CSV Import (Upsert)

All CSV import endpoints use upsert behavior — matching records are updated, new records are created. Re-importing the same CSV is safe and idempotent.

- `POST /import/items` - Bulk import food items from CSV (merge key: name + brand + type + location + purchase_date)
- `POST /import/trips` - Bulk import a grocery trip with items from CSV (trip merge key: trip_date + store_name)
- `POST /import/meals` - Bulk import meals from CSV (merge key: name + made_on)

## Authentication

The API supports optional basic auth. Set both `AUTH_USERNAME` and `AUTH_PASSWORD` environment variables to enable it. When enabled, all endpoints except `GET /health` require credentials.

**When auth is disabled** (default for local dev): all endpoints are open, no credentials needed.

**When auth is enabled**: pass credentials with every request using `-u username:password`:
```bash
# With basic auth enabled
curl -u admin:changeme http://localhost:3000/api/v1/items

# POST with basic auth
curl -u admin:changeme -X POST http://localhost:3000/api/v1/locations \
  -H "Content-Type: application/json" \
  -d '{"name": "pantry"}'

# Health check always works without auth
curl http://localhost:3000/health
```

Browsers will automatically show a login prompt when you visit a protected URL.

## Setup Instructions

### 1. Prerequisites
- Rust (latest stable)
- Docker and Docker Compose
- SQLx CLI: `cargo install sqlx-cli --no-default-features --features postgres`

### 2. Start Database
```bash
docker-compose up -d
```

### 3. Configure Environment
Copy `.env.example` to `.env` and adjust if needed.

### 4. Run Migrations
```bash
sqlx migrate run
```

### 5. Start Server
```bash
cargo run
```

The server will start on `http://localhost:3000`.

## Example Usage

### Create a Location
```bash
curl -X POST http://localhost:3000/api/v1/locations \
  -H "Content-Type: application/json" \
  -d '{"name": "pantry", "description": "Main kitchen pantry"}'
```

### Create a Food Item
```bash
curl -X POST http://localhost:3000/api/v1/items \
  -H "Content-Type: application/json" \
  -d '{
    "type": "produce",
    "name": "Organic Bananas",
    "quantity": 6,
    "unit": "count",
    "price": 3.99,
    "purchase_date": "2024-02-01",
    "location_id": "<location-uuid>"
  }'
```

### Create a Grocery Trip with Items
```bash
curl -X POST http://localhost:3000/api/v1/trips \
  -H "Content-Type: application/json" \
  -d '{
    "trip_date": "2024-02-07",
    "store_name": "Whole Foods",
    "total_spent": 45.67,
    "notes": "Weekly shopping",
    "item_ids": ["<item-uuid-1>", "<item-uuid-2>"]
  }'
```

### List All Items
```bash
curl http://localhost:3000/api/v1/items
```

### Update an Item
```bash
curl -X PUT http://localhost:3000/api/v1/items/<item-uuid> \
  -H "Content-Type: application/json" \
  -d '{
    "type": "produce",
    "name": "Organic Bananas",
    "quantity": 3,
    "unit": "count",
    "price": 3.99,
    "purchase_date": "2024-02-01",
    "location_id": "<location-uuid>"
  }'
```

### Delete an Item
```bash
curl -X DELETE http://localhost:3000/api/v1/items/<item-uuid>
```

### Import Food Items from CSV

Create a file called `items.csv`:
```csv
location,type,brand,name,quantity,unit,price,expiration_date,purchase_date,notes
pantry,produce,,Organic Bananas,6,count,3.99,,2024-02-07,
fridge,dairy,Horizon,Whole Milk,1,gallon,5.99,2024-02-20,2024-02-07,
basement,canned,,Black Beans,4,cans,,,2024-01-15,inventory check
```

Upload it:
```bash
curl -X POST http://localhost:3000/api/v1/import/items -F "file=@items.csv"
```

**Expected response:**
```json
{
  "rows_processed": 3,
  "locations_created": 0,
  "food_items_created": 3,
  "food_items_updated": 0
}
```

### Import a Grocery Trip from CSV

Create a file called `trips.csv`:
```csv
trip_date,store_name,total_spent,trip_notes,location,type,brand,name,quantity,unit,price,expiration_date,purchase_date,item_notes
2024-02-07,Whole Foods,45.67,,pantry,produce,,Organic Bananas,6,count,3.99,,2024-02-07,
2024-02-07,Whole Foods,45.67,,fridge,dairy,Horizon,Whole Milk,1,gallon,5.99,2024-02-20,2024-02-07,
```

Upload it:
```bash
curl -X POST http://localhost:3000/api/v1/import/trips -F "file=@trips.csv"
```

**Expected response:**
```json
{
  "rows_processed": 2,
  "locations_created": 0,
  "food_items_created": 2,
  "food_items_updated": 0,
  "trips_created": 1,
  "trips_updated": 0,
  "trip_items_linked": 2
}
```

Rows with the same `trip_date` and `store_name` are grouped into a single trip. Locations are resolved by name (case-insensitive) and created automatically if they don't exist.

### Import Meals from CSV

Create a file called `meals.csv`:
```csv
name,made_on,servings,last_eaten,rating,ingredients,recipe_link
Chicken Tikka Masala,2024-02-07,6,,loved,"chicken thighs, yogurt, tikka paste, tomatoes, cream, rice",https://example.com/tikka-masala
Pasta Carbonara,2024-02-10,4,2024-02-12,enjoyed,"spaghetti, pancetta, eggs, parmesan",
```

Upload it:
```bash
curl -X POST http://localhost:3000/api/v1/import/meals -F "file=@meals.csv"
```

**Expected response:**
```json
{
  "rows_processed": 2,
  "meals_created": 2,
  "meals_updated": 0
}
```

Re-importing the same CSV will update existing meals instead of creating duplicates (matched by name + made_on).

### Upsert Behavior

All CSV import endpoints use upsert (insert-or-update) behavior. Re-importing the same CSV is safe:
- **First import**: records are created
- **Subsequent imports**: matching records are updated with the new values

Merge keys used to identify existing records:
- **Food items**: name + brand + type + location + purchase_date
- **Grocery trips**: trip_date + store_name
- **Meals**: name + made_on

## Troubleshooting

### Database Connection Errors
- Ensure PostgreSQL is running: `docker-compose ps`
- Check DATABASE_URL in `.env` matches docker-compose.yml credentials

### Migration Errors
- Verify SQLx CLI is installed: `sqlx --version`
- Check migration files are in `migrations/` directory
- Try: `sqlx database drop && sqlx database create && sqlx migrate run`

### Port Already in Use
- Change PORT in `.env` to a different value (e.g., 3001)
- Or kill the process using port 3000: `lsof -ti:3000 | xargs kill`

### CORS Issues (if adding a frontend)
- Add tower-http with "cors" feature to Cargo.toml
- Configure CORS middleware in main.rs

## Development Tips

### Viewing Logs
Set `RUST_LOG=debug` in `.env` for verbose logging.

### Testing with HTTPie (alternative to curl)
```bash
brew install httpie

# GET request
http GET localhost:3000/api/v1/locations

# POST request
http POST localhost:3000/api/v1/locations name=pantry description="Main pantry"
```

### Database Access
Connect directly to PostgreSQL:
```bash
docker-compose exec postgres psql -U grocery_user -d home_inventory
```

### Reset Database
```bash
sqlx database drop
sqlx database create
sqlx migrate run
```

## Project Structure

```
home_inventory/
├── Cargo.toml
├── .env
├── .env.example
├── .gitignore
├── docker-compose.yml
├── README.md
├── IMPLEMENTATION_GUIDE.md (this file)
├── migrations/
│   ├── 20240101000001_create_locations.sql
│   ├── 20240101000002_create_grocery_trips.sql
│   ├── 20240101000003_create_food_items.sql
│   └── 20240101000004_create_trip_items.sql
└── src/
    ├── main.rs
    ├── auth.rs
    ├── config.rs
    ├── error.rs
    ├── routes.rs
    ├── models/
    │   ├── mod.rs
    │   ├── location.rs
    │   ├── food_item.rs
    │   ├── grocery_trip.rs
    │   ├── trip_item.rs
    │   ├── meal.rs
    │   ├── import.rs
    │   └── pagination.rs
    ├── db/
    │   ├── mod.rs
    │   ├── locations.rs
    │   ├── food_items.rs
    │   ├── grocery_trips.rs
    │   ├── trip_items.rs
    │   ├── meals.rs
    │   └── import.rs
    └── handlers/
        ├── mod.rs
        ├── health.rs
        ├── dashboard.rs
        ├── locations.rs
        ├── food_items.rs
        ├── grocery_trips.rs
        ├── meals.rs
        └── import.rs
```
