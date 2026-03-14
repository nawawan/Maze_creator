-- Add up migration script here
ALTER TABLE blogs RENAME COLUMN update_at TO updated_at;
ALTER TABLE blogs ALTER COLUMN updated_at SET DEFAULT now();
ALTER TABLE blogs ALTER COLUMN updated_at SET NOT NULL;