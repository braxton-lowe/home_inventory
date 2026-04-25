CREATE TABLE meals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    made_on DATE NOT NULL,
    servings INTEGER NOT NULL,
    last_eaten DATE,
    rating TEXT NOT NULL CHECK (rating IN ('didnt_enjoy', 'enjoyed', 'loved')),
    ingredients TEXT,
    recipe_link TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_meals_made_on ON meals(made_on);
