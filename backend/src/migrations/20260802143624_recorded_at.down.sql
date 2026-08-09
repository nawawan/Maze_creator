-- Add down migration script here
ALTER TABLE trajectory_lod RENAME COLUMN recorded_ats TO recorted_ats;
ALTER TABLE trajectory_lod RENAME COLUMN heart_rates TO hear_rates;