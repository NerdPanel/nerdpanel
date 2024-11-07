-- Node
CREATE TABLE node (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    fqdn VARCHAR(255) NOT NULL
);

-- Server
CREATE TABLE server (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    node_id INTEGER NOT NULL REFERENCES node(id),
    cpu_limit INTEGER,
    memory_limit INTEGER,
    disk_limit INTEGER
);

-- NodePort
CREATE TABLE node_port (
    id SERIAL PRIMARY KEY,
    node_id INTEGER NOT NULL REFERENCES node(id),
    server_id INTEGER REFERENCES server(id),
    is_primary BOOLEAN NOT NULL,
    ip VARCHAR(45) NOT NULL,
    port INTEGER NOT NULL
);