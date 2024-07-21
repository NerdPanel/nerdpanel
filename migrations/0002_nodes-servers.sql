-- Create the nodes table
CREATE TABLE nodes (
    node_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    node_name VARCHAR(50) NOT NULL,
    cpu INT NOT NULL,
    memory INT NOT NULL,
    disk INT NOT NULL,
    api_url VARCHAR(50) NOT NULL
);

-- Create server status enum
CREATE TYPE server_status AS ENUM ('running', 'stopped', 'starting', 'stopping', 'restarting', 'installing');

-- Create the servers table
CREATE TABLE servers (
    server_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    server_name VARCHAR(50) NOT NULL,
    server_ip VARCHAR(15) NOT NULL,
    server_port INT NOT NULL,
    server_status server_status NOT NULL,
    cpu INT NOT NULL,
    memory INT NOT NULL,
    disk INT NOT NULL,
    node_id UUID NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(node_id),
    owner_id  UUID NOT NULL, 
    FOREIGN KEY (owner_id) REFERENCES users(id)
);  