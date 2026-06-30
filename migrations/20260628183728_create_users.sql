CREATE EXTENSION IF NOT EXISTS pgcrypto;
-- pgcrypto is a PostgreSQL extension that provides cryptographic functions such as:

-- Generating UUIDs
-- Hashing data
-- Encrypting/decrypting data
-- Generating random bytes
CREATE TABLE users (
    id UUID PRIMARY KEY
        DEFAULT gen_random_uuid(),

    email TEXT NOT NULL UNIQUE,

    password_hash TEXT NOT NULL,

    name TEXT NOT NULL,

    created_at TIMESTAMPTZ
        NOT NULL
        DEFAULT NOW(),

    updated_at TIMESTAMPTZ
        NOT NULL
        DEFAULT NOW()
);