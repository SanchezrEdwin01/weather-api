-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS weather (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    city VARCHAR(255) NOT NULL,
    temperature VARCHAR(255) NOT NULL,
    description TEXT,
    humidity VARCHAR(255),
    wind_speed VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
