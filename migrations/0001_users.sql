-- Generate a UUID extension if it doesn't exist
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    pw_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);