-- Add migration script here
ALTER TABLE blogs ADD COLUMN content_key VARCHAR(255) UNIQUE NOT NULL;
ALTER TABLE blogs ADD COLUMN status VARCHAR(10);
ALTER TABLE blogs ADD COLUMN published_at TIMESTAMP;