-- Create locations table
CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert seed data
INSERT INTO locations (name, description) VALUES
    ('pantry', 'Main kitchen pantry'),
    ('fridge', 'Refrigerator'),
    ('freezer', 'Freezer'),
    ('basement', 'Basement storage');
