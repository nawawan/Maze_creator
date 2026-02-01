-- Add migration script here
BEGIN;
ALTER TABLE blogs ADD COLUMN id_uuid uuid;

ALTER TABLE blogs
ALTER COLUMN id_uuid SET NOT NULL;

ALTER TABLE blogs
ALTER COLUMN id_uuid SET DEFAULT gen_random_uuid();

ALTER TABLE blogs
  DROP CONSTRAINT blogs_pkey,
  ADD PRIMARY KEY (id_uuid);
COMMIT;