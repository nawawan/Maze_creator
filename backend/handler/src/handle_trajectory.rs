use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, State};
use std::sync::Arc;
use tracing::error;

use super::helper;
use crate::error::UsecaseError;
use crate::extractor::AuthorizedUser;
use crate::handler::Handler;
use crate::model::activity::ActivityResponse;
use crate::model::trajectory::{Gpx, Trajectory};
use usecase::service::service::Service;
use usecase::service::trajectory::trajectory_service::TrajectoryService;
impl Handler {
    pub async fn upload_gpx(
        user: AuthorizedUser,
        state: State<Arc<Service>>,
        body: Bytes,
    ) -> Result<Json<ActivityResponse>, UsecaseError> {
        if let Err(e) = helper::validate_admin(&user) {
            error!("Permission denied: {}", e.error.message);
            return Err(e);
        }

        let gpx: Gpx = quick_xml::de::from_reader(body.as_ref())
            .map_err(|e| UsecaseError::bad_request(&format!("Failed to parse GPX: {}", e)))?;
        let trajectory = Trajectory::from(gpx);

        let service = state.0.clone();
        let activity = service
            .create_activity_by_trajectory(trajectory.trajectory, user.user.id)
            .await
            .map_err(|e| {
                error!(e.message);
                UsecaseError::internal(&format!("Failed to create activity: {}", e))
            })?;

        Ok(Json(activity.into()))
    }
}
