-- 0001_init: extensions + shared helpers.
--
-- Migrations are plain, forward-only SQL applied in filename order by
-- `sqlx migrate run` (and on boot when RUN_MIGRATIONS_ON_START=true).

-- citext -> case-insensitive unique usernames / emails without LOWER() everywhere.
CREATE EXTENSION IF NOT EXISTS citext;

-- gen_random_uuid() is in core PG13+, but pgcrypto is handy for future needs.
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Reusable trigger to keep `updated_at` honest.
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
