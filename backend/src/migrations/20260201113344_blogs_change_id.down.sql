-- Add down migration script here

ALTER TABLE blogs RENAME COLUMN id TO id_uuid;

ALTER TABLE blogs ADD COLUMN id INT DEFAULT unique_rowid();