use crate::model::trajectory::Coordinate;

use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6378137.0; // in meters

fn convert_coordinates_to_web_mercator(coordinates: &Vec<Coordinate>) -> Vec<(f64, f64)> {
    coordinates
        .iter()
        .map(|coord| {
            let x = coord.longitude.to_radians() * EARTH_RADIUS;
            let y = (coord.latitude.to_radians() + PI / 4.0).tan().abs().ln() * EARTH_RADIUS;
            (x, y)
        })
        .collect()
}

fn convert_coordinates_to_equidistant_cylindrical(
    coordinates: &Vec<Coordinate>,
) -> Vec<(f64, f64)> {
    let lon0 = coordinates
        .first()
        .map_or(0.0, |coord| coord.longitude.to_radians());
    let lat0 = coordinates
        .first()
        .map_or(0.0, |coord| coord.latitude.to_radians());
    coordinates
        .iter()
        .map(|coord| {
            let x = (coord.longitude.to_radians() - lon0) * EARTH_RADIUS * lat0.cos();
            let y = (coord.latitude.to_radians() - lat0) * EARTH_RADIUS;
            (x, y)
        })
        .collect()
}

// calculater the distance (meter) between two coordinates using haversine formula
pub fn haversine_distance(coord1: &Coordinate, coord2: &Coordinate) -> f64 {
    let lat1_rad = coord1.latitude.to_radians();
    let lat2_rad = coord2.latitude.to_radians();
    let delta_lat = (coord2.latitude - coord1.latitude).to_radians();
    let delta_lon = (coord2.longitude - coord1.longitude).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS * c
}

// calculate the picked up index of coordinates using Douglas-Peucker algorithm
pub fn douglas_peucker_simplify(coordinates: &Vec<Coordinate>, epsilon: f64) -> Vec<usize> {
    if coordinates.len() < 3 {
        return (0..coordinates.len()).collect();
    }

    let meter_coordinates = convert_coordinates_to_web_mercator(coordinates);
    douglas_peucker(&meter_coordinates, epsilon)
}

fn douglas_peucker(coordinate: &Vec<(f64, f64)>, epsilon: f64) -> Vec<usize> {
    // Implementation for Douglas-Peucker algorithm
    if coordinate.len() < 3 {
        return (0..coordinate.len()).collect();
    }

    let mut stack = Vec::<(usize, usize)>::new();
    stack.push((0, coordinate.len() - 1));

    let threshold = epsilon * epsilon; // Use squared distance for comparison
    let mut result_indices = Vec::<usize>::new();
    result_indices.push(0);
    result_indices.push(coordinate.len() - 1);

    while let Some((l, r)) = stack.pop() {
        if r - l == 1 {
            continue;
        }
        let (lx, ly) = coordinate[l];
        let (rx, ry) = coordinate[r];

        let base = (rx - lx).powi(2) + (ry - ly).powi(2);
        let current_threshold = threshold * base;

        let mut max_distance = 0.0;
        let mut index: usize = l;
        for i in (l + 1)..r {
            let (px, py) = coordinate[i];
            let area = ((rx - lx) * (py - ly) - (px - lx) * (ry - ly)).powi(2);

            if area > current_threshold && area > max_distance {
                max_distance = area;
                index = i;
            }
        }

        if index != l {
            stack.push((l, index));
            stack.push((index, r));
            result_indices.push(index);
        }
    }

    result_indices.sort_unstable();
    result_indices
}
