-- 0004_likes: one row per (user, post). Composite PK makes likes idempotent.

CREATE TABLE likes (
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id    UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (user_id, post_id)
);

-- Count likes for a post / list likers.
CREATE INDEX likes_post_id ON likes (post_id);
