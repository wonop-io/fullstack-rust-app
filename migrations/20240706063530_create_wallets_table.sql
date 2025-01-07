-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS
    "wallets" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        user_id UUID NOT NULL UNIQUE,
        encrypted_private_key TEXT NOT NULL,
        address VARCHAR(42) NOT NULL,
        balance NUMERIC(28, 18) NOT NULL DEFAULT 0,
        salt BYTEA NOT NULL,
        token_decimals SMALLINT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT NOW(),
        updated_at TIMESTAMPTZ DEFAULT NOW(),
        FOREIGN KEY (user_id) REFERENCES auth_users(id) ON DELETE CASCADE
    );

CREATE INDEX wallets_user_id_idx ON wallets (user_id);
CREATE INDEX wallets_address_idx ON wallets (address);
