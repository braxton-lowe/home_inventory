ALTER TABLE food_items
  ADD COLUMN active BOOLEAN NOT NULL DEFAULT TRUE,
  ADD COLUMN consumed_at TIMESTAMPTZ;

CREATE INDEX idx_food_items_active ON food_items (active);
