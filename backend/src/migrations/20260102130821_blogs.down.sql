-- Add migration script here
ALTER TABLE blogs 
  DROP CONSTRAINT blogs_pkey,
  ADD PRIMARY KEY (id);

ALTER TABLE blogs DROP COLUMN id_uuid;