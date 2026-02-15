-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    password TEXT NOT NULL
);
