-- Your SQL goes here
CREATE TABLE IF NOT EXISTS groups (
    internal_id BIGINT NOT NULL PRIMARY KEY,
    service_namespace TEXT NOT NULL,
    group_external_id BIGINT NOT NULL,
    group_name TEXT NOT NULL,
    UNIQUE (service_namespace, group_external_id),
    UNIQUE (service_namespace, group_name)
);

CREATE TABLE IF NOT EXISTS group_memberships (
    group_internal_id BIGINT NOT NULL,
    user_internal_id BIGINT NOT NULL,
    PRIMARY KEY (group_internal_id, user_internal_id),
    FOREIGN KEY (group_internal_id) REFERENCES groups(internal_id) ON DELETE CASCADE,
    FOREIGN KEY (user_internal_id) REFERENCES users(internal_id) ON DELETE CASCADE
);