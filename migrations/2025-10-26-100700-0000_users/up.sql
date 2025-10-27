-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    internal_id BIGINT NOT NULL PRIMARY KEY,
    user_namespace TEXT NOT NULL,
    external_id TEXT NOT NULL,
    UNIQUE(user_namespace, external_id)
);