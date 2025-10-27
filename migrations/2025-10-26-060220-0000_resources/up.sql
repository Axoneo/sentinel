-- Your SQL goes here
CREATE TABLE IF NOT EXISTS resources (
    internal_id BIGINT NOT NULL PRIMARY KEY,
    service_namespace TEXT NOT NULL,
    resource_name TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    parent_internal_id BIGINT
);

-- FOR SOFT LINKS BETWEEN RESOURCES
CREATE TABLE IF NOT EXISTS links (
    link_internal_id BIGINT NOT NULL PRIMARY KEY,
    target_internal_id BIGINT NOT NULL,
    FOREIGN KEY (link_internal_id) REFERENCES resources(internal_id),
    FOREIGN KEY (target_internal_id) REFERENCES resources(internal_id)
);