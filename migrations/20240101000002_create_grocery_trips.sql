-- Create grocery_trips table
CREATE TABLE grocery_trips (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trip_date DATE NOT NULL,
    store_name TEXT NOT NULL,
    total_spent DECIMAL(10, 2),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for querying by date
CREATE INDEX idx_grocery_trips_trip_date ON grocery_trips(trip_date);
