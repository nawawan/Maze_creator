use crate::errors::app_error::AppError;
use crate::model::activity::Activity;
use crate::model::trajectory::{LodTrajectory, RawTrajectory};
use crate::service::service::Service;
use crate::service::trajectory::helper;

use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

#[async_trait]
pub trait TrajectoryService {
    // async fn upload_trajectory(&self, trajectory: Trajectory) -> Result<Trajectory, AppError>;
    // async fn get_trajectory(&self, activity_id: String, zoom_level: i32) -> Result<Trajectory, AppError>;
    // async fn delete_trajectory(&self, activity_id: String) -> Result<(), AppError>;
    async fn create_activity_by_trajectory(
        &self,
        trajectory: RawTrajectory,
        user_id: Uuid,
    ) -> Result<Activity, AppError>;
    async fn get_activity(&self, activity_id: String) -> Result<Activity, AppError>;
    async fn list_activities(&self, user_id: String) -> Vec<Activity>;
}

#[async_trait]
impl TrajectoryService for Service {
    async fn create_activity_by_trajectory(
        &self,
        trajectory: RawTrajectory,
        user_id: Uuid,
    ) -> Result<Activity, AppError> {
        let sum_distance = helper::calculate_distance_sum(&trajectory.coordinates);
        let duration = helper::calculate_duration(&trajectory.recorded_ats);
        let elevation = helper::calculate_elevation_sum(&trajectory.elevations);

        let mut trajectories = Vec::<LodTrajectory>::new();
        for i in 1..4 {
            trajectories.push(helper::thin_out_raw_trajectory_by_lod(&trajectory, i));
        }

        let thin_trajectory = trajectories[0].coordinates.clone();

        let activity = Activity {
            id: 0,
            user_id: user_id,
            name: trajectory.name,
            distance: sum_distance,
            duration: duration,
            elevation_gain: elevation,
            start_time: trajectory.started_at,
            thin_trajectory: None,
        };

        let mut tx = self.repository.create_transaction().await?;

        let mut activity = self.repository.create_activity(&mut tx, activity).await?;

        for mut lod_trajectory in trajectories {
            lod_trajectory.activity_id = activity.id;
            self.repository
                .create_trajectory(&mut tx, lod_trajectory)
                .await?;
        }
        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction for creating activity: {e}");
            AppError::internal(Some("Transaction commit failed"))
        })?;
        activity.thin_trajectory = Some(thin_trajectory);

        Ok(activity)
    }
    async fn get_activity(&self, activity_id: String) -> Result<Activity, AppError> {
        let activity = self.repository.get_activity(activity_id).await?;
        Ok(activity)
    }
    async fn list_activities(&self, user_id: String) -> Vec<Activity> {
        self.repository.list_activities(user_id).await
    }
}
