-- 0005_comments: flat comments on a post (no threading yet).

CREATE TABLE comments (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id    UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    author_id  UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    body       TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT comments_body_len CHECK (char_length(body) BETWEEN 1 AND 1000)
);

-- List a post's comments oldest-first, keyset-paginated.
CREATE INDEX comments_post_created_at ON comments (post_id, created_at ASC, id ASC);

CREATE TRIGGER comments_set_updated_at
    BEFORE UPDATE ON comments
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
