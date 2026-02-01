-- Add migration script here
ALTER TABLE blogs DROP COLUMN content_key;
ALTER TABLE blogs DROP COLUMN status;
ALTER TABLE blogs DROP COLUMN published_at;