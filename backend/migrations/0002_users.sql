-- 0002_users: accounts. Supports both email/password and OAuth-only users.

CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username      CITEXT NOT NULL UNIQUE,
    email         CITEXT NOT NULL UNIQUE,
    -- NULL for OAuth-only accounts (e.g. Google Sign-In, no local password).
    password_hash TEXT,
    -- OAuth linkage. One column per provider keeps lookups trivially indexable;
    -- add `apple_id` etc. in a later migration when the provider is added.
    google_id     TEXT UNIQUE,
    display_name  TEXT,
    bio           TEXT NOT NULL DEFAULT '',
    avatar_url    TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT users_username_len CHECK (char_length(username) BETWEEN 3 AND 30),
    -- Every account must be reachable by *some* credential.
    CONSTRAINT users_has_credential CHECK (password_hash IS NOT NULL OR google_id IS NOT NULL)
);

CREATE TRIGGER users_set_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
