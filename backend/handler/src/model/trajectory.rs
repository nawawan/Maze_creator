use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use usecase::model::trajectory::{Coordinate, RawTrajectory};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Gpx {
    pub metadata: Metadata,
    #[serde(rename = "trk")]
    pub track: Track,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub time: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: String,
    #[serde(rename = "trkseg")]
    pub track_segments: TrackSegment,
}

#[derive(Deserialize, Debug)]
pub struct TrackSegment {
    pub trkpt: Vec<TrackPoint>,
}

#[derive(Deserialize, Debug)]
pub struct TrackPoint {
    #[serde(rename = "@lat")]
    pub latitude: f64,
    #[serde(rename = "@lon")]
    pub longitude: f64,
    #[serde(rename = "ele")]
    pub elevation: f64,
    pub time: DateTime<Utc>,
    pub extensions: Option<Extensions>,
}

#[derive(Deserialize, Debug)]
pub struct Extensions {
    #[serde(rename = "TrackPointExtension")]
    pub track_point_extension: Option<TrackPointExtension>,
}

#[derive(Deserialize, Debug)]
pub struct TrackPointExtension {
    #[serde(rename = "hr")]
    pub heart_rate: Option<i64>,
    #[serde(rename = "atemp")]
    pub ambient_temperature: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LineString {
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<[f64; 2]>,
}

pub struct Trajectory {
    pub trajectory: RawTrajectory,
}

impl From<Gpx> for Trajectory {
    fn from(gpx: Gpx) -> Self {
        let coordinates = gpx
            .track
            .track_segments
            .trkpt
            .iter()
            .map(|trpt| usecase::model::trajectory::Coordinate {
                latitude: trpt.latitude,
                longitude: trpt.longitude,
            })
            .collect::<Vec<_>>();

        let elevations = gpx
            .track
            .track_segments
            .trkpt
            .iter()
            .map(|trpt| trpt.elevation)
            .collect::<Vec<_>>();

        let heart_rates = gpx
            .track
            .track_segments
            .trkpt
            .iter()
            .filter_map(|trpt| {
                trpt.extensions.as_ref().and_then(|ext| {
                    ext.track_point_extension
                        .as_ref()
                        .and_then(|tpe| tpe.heart_rate)
                })
            })
            .collect::<Vec<_>>();

        let recorded_ats = gpx
            .track
            .track_segments
            .trkpt
            .iter()
            .map(|trpt| trpt.time.naive_local())
            .collect::<Vec<_>>();

        Trajectory {
            trajectory: RawTrajectory {
                name: gpx.track.name.clone(),
                trajectory_type: gpx.track.activity_type.clone(),
                coordinates,
                elevations,
                heart_rates,
                recorded_ats,
                started_at: gpx.metadata.time.naive_local(),
            },
        }
    }
}


impl From<Vec<Coordinate>> for LineString {
    fn from(coords: Vec<Coordinate>) -> Self {
        Self {
            geometry_type: "LineString".to_string(),
            coordinates: coords
                .into_iter()
                .map(|c| [c.longitude, c.latitude])
                .collect(),
        }
    }
}