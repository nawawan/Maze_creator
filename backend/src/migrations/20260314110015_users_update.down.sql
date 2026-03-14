-- Add down migration script here
ALTER TABLE users ALTER COLUMN salt DROP NOT NULL;