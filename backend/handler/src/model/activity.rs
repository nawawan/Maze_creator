use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use usecase::model::activity::Activity;

use crate::model::trajectory::LineString;

#[derive(Serialize, Deserialize, Clone)]
pub struct ActivityResponse {
    pub id: i64,
    pub name: String,
    pub distance: f64,
    pub duration: i64,
    pub elevation_gain: f64,
    pub start_time: NaiveDateTime,
    pub trajectory: Option<LineString>,
}

impl From<Activity> for ActivityResponse {
    fn from(activity: Activity) -> Self {
        Self {
            id: activity.id,
            name: activity.name,
            distance: activity.distance,
            duration: activity.duration,
            elevation_gain: activity.elevation_gain,
            start_time: activity.start_time,
            trajectory: activity
                .thin_trajectory
                .map_or(None, |coord| Some(coord.into())),
        }
    }
}
