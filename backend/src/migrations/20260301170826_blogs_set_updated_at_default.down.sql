-- Add down migration script here
ALTER TABLE blogs ALTER COLUMN updated_at DROP NOT NULL;
ALTER TABLE blogs ALTER COLUMN updated_at DROP DEFAULT;
ALTER TABLE blogs RENAME COLUMN updated_at TO update_at;