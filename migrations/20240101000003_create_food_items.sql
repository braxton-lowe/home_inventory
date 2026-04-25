-- Create food_items table
CREATE TABLE food_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type TEXT NOT NULL,
    brand TEXT,
    name TEXT NOT NULL,
    quantity DECIMAL(10, 2) NOT NULL,
    unit TEXT NOT NULL,
    price DECIMAL(10, 2),
    expiration_date DATE,
    purchase_date DATE NOT NULL,
    notes TEXT,
    location_id UUID NOT NULL REFERENCES locations(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for common queries
CREATE INDEX idx_food_items_location_id ON food_items(location_id);
CREATE INDEX idx_food_items_type ON food_items(type);
CREATE INDEX idx_food_items_expiration_date ON food_items(expiration_date);
