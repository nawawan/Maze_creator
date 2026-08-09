use chrono::{DateTime, NaiveDateTime, Utc};

use super::geo::{douglas_peucker_simplify, haversine_distance};
use crate::model::trajectory::{Coordinate, LodTrajectory, RawTrajectory};

pub fn thin_out_raw_trajectory_by_lod(trajectory: &RawTrajectory, lod: usize) -> LodTrajectory {
    let epsiron = match lod {
        1 => 500.0,
        2 => 100.0,
        3 => 20.0,
        _ => 1.0,
    };

    let picked_up_indices = douglas_peucker_simplify(&trajectory.coordinates, epsiron);
    let cap = picked_up_indices.len();

    let mut thinned_coordinates = Vec::with_capacity(cap);
    let mut thinned_elevations = Vec::with_capacity(cap);
    let mut thinned_heart_rates = Vec::with_capacity(cap);
    let mut thinned_recorded_ats = Vec::with_capacity(cap);

    picked_up_indices.iter().for_each(|&idx| {
        thinned_coordinates.push(trajectory.coordinates[idx].clone());
        thinned_elevations.push(trajectory.elevations[idx]);
        thinned_heart_rates.push(trajectory.heart_rates[idx]);
        thinned_recorded_ats.push(trajectory.recorded_ats[idx]);
    });

    let (zoom_from, zoom_to) = match lod {
        1 => (1, 2),
        2 => (3, 4),
        3 => (5, 6),
        _ => (7, 8),
    };

    LodTrajectory {
        id: 0,
        activity_id: 0,
        name: trajectory.name.clone(),
        trajectory_type: trajectory.trajectory_type.clone(),
        coordinates: thinned_coordinates,
        elevations: thinned_elevations,
        heart_rates: thinned_heart_rates,
        recorded_ats: thinned_recorded_ats,
        started_at: trajectory.started_at,
        zoom_from,
        zoom_to,
    }
}

// calculate the total distance (meter) of a trajectory represented by a vector of coordinates
pub fn calculate_distance_sum(coordinates: &Vec<Coordinate>) -> f64 {
    coordinates
        .windows(2)
        .map(|pair| {
            let prev = &pair[0];
            let curr = &pair[1];
            haversine_distance(prev, curr)
        })
        .sum()
}

pub fn calculate_elevation_sum(elevations: &Vec<f64>) -> f64 {
    elevations
        .windows(2)
        .map(|pair| {
            let prev = &pair[0];
            let curr = &pair[1];

            (curr - prev).max(0.0)
        })
        .sum()
}

pub fn calculate_duration(recorded_ats: &Vec<NaiveDateTime>) -> i64 {
    if recorded_ats.len() < 2 {
        return 0;
    }

    let start = recorded_ats.first().unwrap();
    let end = recorded_ats.last().unwrap();

    (end.and_utc().timestamp() - start.and_utc().timestamp()) / 60
}
