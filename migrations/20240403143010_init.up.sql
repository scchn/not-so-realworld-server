CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE COLLATION IF NOT EXISTS case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);

CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    username TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    email TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
