-- Add up migration script here
ALTER TABLE trajectory_lod RENAME COLUMN recorted_ats TO recorded_ats;
ALTER TABLE trajectory_lod RENAME COLUMN hear_rates TO heart_rates;