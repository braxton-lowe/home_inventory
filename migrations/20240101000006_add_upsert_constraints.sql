-- Add unique indexes to support ON CONFLICT upsert behavior for CSV imports.

CREATE UNIQUE INDEX idx_food_items_upsert_key
  ON food_items (name, COALESCE(brand, ''), type, location_id, purchase_date);

CREATE UNIQUE INDEX idx_grocery_trips_upsert_key
  ON grocery_trips (trip_date, store_name);

CREATE UNIQUE INDEX idx_meals_upsert_key
  ON meals (name, made_on);
