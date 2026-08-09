-- Add up migration script here
CREATE TABLE activity (
    id SERIAL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users(id),
    name VARCHAR(100) NOT NULL,
    started_at TIMESTAMP NOT NULL,
    distance FLOAT NOT NULL DEFAULT 0,
    elevation int NOT NULL DEFAULT 0,
    duration int NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE trajectory_raw (
    id SERIAL PRIMARY KEY,
    activity_id int NOT NULL REFERENCES activity(id),
    trajectory GEOMETRY NOT NULL,
    recorted_ats TIMESTAMP[] NOT NULL DEFAULT ARRAY[]::TIMESTAMP[],
    elevations float[] NOT NULL DEFAULT ARRAY[]::FLOAT[],
    hear_rates int[] NOT NULL DEFAULT ARRAY[]::INT[],
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE trajectory_lod(
    id SERIAL PRIMARY KEY,
    activity_id int NOT NULL REFERENCES activity(id),
    trajectory GEOMETRY NOT NULL,
    recorted_ats TIMESTAMP[] NOT NULL DEFAULT ARRAY[]::TIMESTAMP[],
    elevations float[] NOT NULL DEFAULT ARRAY[]::FLOAT[],
    hear_rates int[] NOT NULL DEFAULT ARRAY[]::INT[],
    zoom_from INT NOT NULL,
    zoom_to INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);