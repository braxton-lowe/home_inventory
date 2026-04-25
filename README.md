=======
# Home Inventory

A REST API for tracking grocery purchases and food inventory, built with Rust, Axum, SQLx, and PostgreSQL.

## Features

- Track food items with location, purchase date, expiration, quantity, and price
- Organize items by storage location (pantry, fridge, basement, etc.)
- Record grocery shopping trips and link them to purchased items
- Track meals with ratings, ingredients, and recipe links
- Full CRUD operations for locations, items, trips, and meals
- Bulk import food items or grocery trips via CSV file upload
- Normalized database schema with proper relationships

## Quick Start

### Prerequisites

- Rust (latest stable)
- Docker and Docker Compose
- SQLx CLI: `cargo install sqlx-cli --no-default-features --features postgres`

### Setup

1. **Start the database:**
   ```bash
   docker-compose up -d
   ```

2. **Run migrations:**
   ```bash
   sqlx migrate run
   ```

3. **Start the server:**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:3000`.

### Connecting to PostgreSQL

**From inside the Docker container:**
```bash
# First, exec into the container
docker exec -it home_inventory-postgres-1 bash

# Then connect to the database
psql -U grocery_user -d home_inventory
```

**From your host machine:**
```bash
psql -h localhost -p 5432 -U grocery_user -d home_inventory
```

**Password:** `dev_password` (from docker-compose.yml)

**Useful PostgreSQL commands:**
```sql
-- List all tables
\dt

-- Describe a table structure
\d food_items

-- View all locations
SELECT * FROM locations;

-- Count items by location
SELECT l.name, COUNT(fi.id) as item_count
FROM locations l
LEFT JOIN food_items fi ON l.id = fi.location_id
GROUP BY l.name;

-- Exit psql
\q
```

### Accessing Your Data

There are two ways to interact with the server:

- **Dashboard (browser)** — visit `http://localhost:3000/` for an HTML dashboard with tables, search, pagination, and sorting across all 4 data types (items, locations, trips, meals). This is the easiest way to browse your data.

- **JSON API (programmatic)** — use the REST endpoints (`/api/v1/items`, `/api/v1/locations`, etc.) with curl, scripts, or other applications. Use this for creating, updating, deleting records, and CSV imports.

Both exist simultaneously. The dashboard is just a JS client that calls the same API endpoints under the hood. You don't need to pick one or the other — they serve different purposes.

For details on how the dashboard JavaScript works, see [HOW_DASHBOARD_JS_WORKS.md](HOW_DASHBOARD_JS_WORKS.md).

**Health Check:**
```
http://localhost:3000/health
```
Returns `{"status":"ok"}` if the server is running correctly.

## API Documentation

See [IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md) for detailed API documentation, examples, and troubleshooting.

### Quick Examples

**Create a location:**
```bash
curl -X POST http://localhost:3000/api/v1/locations \
  -H "Content-Type: application/json" \
  -d '{"name": "pantry", "description": "Main kitchen pantry"}'
```

**List all items:**
```bash
curl http://localhost:3000/api/v1/items
```

**Health check:**
```bash
curl http://localhost:3000/health
```

**Create a meal:**
```bash
curl -X POST http://localhost:3000/api/v1/meals \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Chicken Tikka Masala",
    "made_on": "2024-02-07",
    "servings": 6,
    "rating": "loved",
    "ingredients": "chicken thighs, yogurt, tikka paste, canned tomatoes, cream, rice",
    "recipe_link": "https://example.com/tikka-masala"
  }'
```

**List all meals:**
```bash
curl http://localhost:3000/api/v1/meals
```

**Update a meal** (e.g. record that the family ate it again):
```bash
curl -X PUT http://localhost:3000/api/v1/meals/<meal-uuid> \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Chicken Tikka Masala",
    "made_on": "2024-02-07",
    "servings": 6,
    "last_eaten": "2024-02-10",
    "rating": "loved",
    "ingredients": "chicken thighs, yogurt, tikka paste, canned tomatoes, cream, rice",
    "recipe_link": "https://example.com/tikka-masala"
  }'
```

**Delete a meal:**
```bash
curl -X DELETE http://localhost:3000/api/v1/meals/<meal-uuid>
```

**Import food items from CSV:**
```bash
curl -X POST http://localhost:3000/api/v1/import/items -F "file=@items.csv"
```

**Import a grocery trip from CSV:**
```bash
curl -X POST http://localhost:3000/api/v1/import/trips -F "file=@trips.csv"
```

See [IMPLEMENTATION_GUIDE.md](IMPLEMENTATION_GUIDE.md) for CSV format details and more examples.

## Project Structure

- `src/models/` - Data structures and DTOs
- `src/db/` - Database queries
- `src/handlers/` - HTTP request handlers
- `src/routes.rs` - Route definitions
- `migrations/` - Database migrations

## License

MIT
>>>>>>> bf9b36b (Initial commit)
