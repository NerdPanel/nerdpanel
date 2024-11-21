-- Node
CREATE TABLE node (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    fqdn VARCHAR(255) NOT NULL
);

-- EnvVar
CREATE TYPE env_var_type AS (
    key TEXT,
    value TEXT
);

-- Image
CREATE TYPE image_type AS (
    name TEXT,
    tag TEXT
);

-- platform
CREATE TABLE platform (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    images image_type[] NOT NULL,
    startup_command TEXT NOT NULL,
    installer_image VARCHAR(255) NOT NULL,
    env_vars env_var_type[] NOT NULL
);

-- Server
CREATE TABLE server (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    node_id INTEGER NOT NULL REFERENCES node(id),

    cpu_limit INTEGER,
    memory_limit INTEGER,
    disk_limit INTEGER,

    platform_id INTEGER NOT NULL REFERENCES platform(id),
    image VARCHAR(255) NOT NULL,
    startup_command TEXT NOT NULL,
    env_vars env_var_type[] NOT NULL

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
