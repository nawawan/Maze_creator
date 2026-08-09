-- Add down migration script here
ALTER TABLE activity ALTER COLUMN elevation TYPE INT USING elevation::INT;
