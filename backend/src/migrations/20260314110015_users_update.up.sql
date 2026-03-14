-- Add up migration script here
ALTER TABLE users ALTER COLUMN salt SET NOT NULL;