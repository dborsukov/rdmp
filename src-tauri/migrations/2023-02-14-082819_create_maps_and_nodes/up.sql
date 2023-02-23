CREATE TABLE IF NOT EXISTS maps (
    uuid VARCHAR NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS nodes (
    uuid VARCHAR NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    node_type VARCHAR NOT NULL,
    parent_node VARCHAR,
    roadmap_uuid VARCHAR NOT NULL,
    FOREIGN KEY (roadmap_uuid) REFERENCES maps (uuid) ON DELETE CASCADE
);
