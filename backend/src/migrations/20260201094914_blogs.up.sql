-- Add migration script here
ALTER TABLE blogs ADD COLUMN content_key VARCHAR(255), status VARCHAR(10) UNIQUE NOT NULL;