-- Add down migration script here
ALTER TABLE trajectory_lod
    ALTER COLUMN zoom_to TYPE int,
    ALTER COLUMN zoom_from TYPE int,
    ALTER COLUMN heart_rates TYPE int[],
    ALTER COLUMN activity_id TYPE int,
    ALTER COLUMN id TYPE int;

ALTER TABLE trajectory_raw
    ALTER COLUMN hear_rates TYPE int[],
    ALTER COLUMN activity_id TYPE int,
    ALTER COLUMN id TYPE int;

ALTER TABLE activity
    ALTER COLUMN duration TYPE int,
    ALTER COLUMN id TYPE int;
