-- Add up migration script here
ALTER TABLE blogs DROP COLUMN id;

ALTER TABLE blogs RENAME COLUMN id_uuid TO id;