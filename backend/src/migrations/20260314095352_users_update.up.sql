-- Add up migration script here
ALTER TABLE users  ADD COLUMN IF NOT EXISTS salt_second TEXT;
ALTER TABLE users DROP COLUMN IF EXISTS salt;
ALTER TABLE users RENAME COLUMN salt_second TO salt;