-- Create trip_items junction table
CREATE TABLE trip_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trip_id UUID NOT NULL REFERENCES grocery_trips(id) ON DELETE CASCADE,
    food_item_id UUID NOT NULL REFERENCES food_items(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(trip_id, food_item_id)
);

-- Indexes for junction table queries
CREATE INDEX idx_trip_items_trip_id ON trip_items(trip_id);
CREATE INDEX idx_trip_items_food_item_id ON trip_items(food_item_id);
