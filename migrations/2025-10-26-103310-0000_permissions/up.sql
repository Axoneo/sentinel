-- Your SQL goes here
CREATE TABLE IF NOT EXISTS permissions (
    service_namespace TEXT NOT NULL,
    resource_internal_id BIGINT NOT NULL,
    subject_internal_id BIGINT NOT NULL,
    subject_type TEXT NOT NULL,
    permission_bits BIGINT NOT NULL,
    PRIMARY KEY (service_namespace, resource_internal_id, subject_internal_id, subject_type),
    FOREIGN KEY (resource_internal_id) REFERENCES resources(internal_id) ON DELETE CASCADE,
    FOREIGN KEY (subject_internal_id) REFERENCES users(internal_id) ON DELETE CASCADE
);

-- 1 - x
-- 2 - w
-- 4 - r