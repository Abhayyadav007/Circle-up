-- 0003_posts: image posts. Bytes live in R2; we only store the object key.

CREATE TABLE posts (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id  UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- R2 object key, e.g. "posts/<author_uuid>/<uuid>.jpg". Resolve to a URL via
    -- StorageService::get_url — never store a full URL (provider may change).
    image_key  TEXT NOT NULL,
    caption    TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT posts_caption_len CHECK (char_length(caption) <= 2200)
);

-- Home/explore feed: newest-first keyset pagination on (created_at, id).
CREATE INDEX posts_created_at_id_desc ON posts (created_at DESC, id DESC);
-- Profile grid: a user's posts, newest first.
CREATE INDEX posts_author_created_at ON posts (author_id, created_at DESC, id DESC);

CREATE TRIGGER posts_set_updated_at
    BEFORE UPDATE ON posts
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
