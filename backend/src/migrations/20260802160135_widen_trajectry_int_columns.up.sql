-- Add up migration script here
ALTER TABLE activity
    ALTER COLUMN id TYPE bigint,
    ALTER COLUMN duration TYPE bigint;

ALTER TABLE trajectory_raw
    ALTER COLUMN id TYPE bigint,
    ALTER COLUMN activity_id TYPE bigint,
    ALTER COLUMN hear_rates TYPE bigint[];

ALTER TABLE trajectory_lod
    ALTER COLUMN id TYPE bigint,
    ALTER COLUMN activity_id TYPE bigint,
    ALTER COLUMN heart_rates TYPE bigint[],
    ALTER COLUMN zoom_from TYPE bigint,
    ALTER COLUMN zoom_to TYPE bigint;
