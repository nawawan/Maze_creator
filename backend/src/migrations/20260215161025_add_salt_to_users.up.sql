-- Add up migration script here
ALTER TABLE users ADD COLUMN salt UUID NOT NULL;