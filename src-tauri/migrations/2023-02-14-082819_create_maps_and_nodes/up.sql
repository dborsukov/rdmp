CREATE TABLE IF NOT EXISTS maps (
    uuid VARCHAR NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS nodes (
    uuid VARCHAR NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    is_main_node BOOLEAN NOT NULL,
    node_order INTEGER NOT NULL,
    parent_node VARCHAR,
    roadmap_uuid VARCHAR NOT NULL,
    FOREIGN KEY (roadmap_uuid) REFERENCES maps (uuid) ON DELETE CASCADE
);