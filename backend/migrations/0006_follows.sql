-- 0006_follows: directed follow graph. (follower_id, followee_id) unique.

CREATE TABLE follows (
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followee_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (follower_id, followee_id),
    CONSTRAINT follows_no_self CHECK (follower_id <> followee_id)
);

-- "Who follows me" (followers list, follower counts).
CREATE INDEX follows_followee_created_at ON follows (followee_id, created_at DESC);
-- "Who do I follow" powers the home feed subquery + following list.
CREATE INDEX follows_follower_created_at ON follows (follower_id, created_at DESC);
