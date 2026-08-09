use async_trait::async_trait;
use geo_types::{Coord, Geometry, LineString};
use geozero::wkb;
use tracing::error;

use crate::repository::Repository;
use usecase::errors::repo_error::RepoError;
use usecase::model::{activity::Activity, trajectory::LodTrajectory};
use usecase::repository::trajectory::TrajectoryRepository;

#[async_trait]
impl TrajectoryRepository for Repository {
    // Implementation for each method would go here
    async fn create_trajectory(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        trajectory: LodTrajectory,
    ) -> Result<LodTrajectory, RepoError> {
        // I hate this conversion
        // Convert the domain model before opening a transaction.
        // Geometry conversion and WKB encoding may be CPU-intensive,
        // so they should not extend the transaction lifetime.
        let line_string = LineString::new(
            trajectory
                .coordinates
                .iter()
                .map(|coord| Coord {
                    x: coord.longitude,
                    y: coord.latitude,
                })
                .collect(),
        );
        sqlx::query!(
            r#"
            INSERT INTO trajectory_lod (activity_id, trajectory, recorded_ats, elevations, heart_rates, zoom_from, zoom_to)
            VALUES($1, ST_SetSRID($2::geometry, 4326), $3, $4, $5, $6, $7);
            "#,
            trajectory.activity_id,
            wkb::Encode(Geometry::LineString(line_string)) as _,
            &trajectory.recorded_ats,
            &trajectory.elevations,
            &trajectory.heart_rates,
            trajectory.zoom_from,
            trajectory.zoom_to,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            error!("Failed to create trajectory: {}", e);
            RepoError::Internal("Failed to create trajectory".to_string())
        })?;
        Ok(trajectory)
    }
    async fn upload_trajectory(
        &self,
        trajectory: LodTrajectory,
    ) -> Result<LodTrajectory, RepoError> {
        // Implementation for uploading a trajectory
        Ok(trajectory)
    }
    async fn get_trajectory(
        &self,
        activity_id: i64,
        zoom_level: i32,
    ) -> Result<LodTrajectory, RepoError> {
        // Implementation for getting a trajectory
        Err(RepoError::NotFound(format!("a")))
    }

    async fn create_activity(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        mut activity: Activity,
    ) -> Result<Activity, RepoError> {
        // Implementation for creating an activity
        let record = sqlx::query!(
            r#"INSERT INTO activity (user_id, name, started_at, distance, elevation, duration) 
            VALUES($1, $2, $3, $4, $5, $6) 
            RETURNING id"#,
            activity.user_id,
            activity.name,
            activity.start_time,
            activity.distance,
            activity.elevation_gain,
            activity.duration,
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| {
            error!("Failed to create activity: {}", e);
            RepoError::Internal("Failed to create activity".to_string())
        })?;
        activity.id = record.id;
        Ok(activity)
    }
    async fn get_activity(&self, activity_id: String) -> Result<Activity, RepoError> {
        // Implementation for getting an activity
        Err(RepoError::NotFound(format!(
            "Activity with id: {} not found",
            activity_id
        )))
    }
    async fn list_activities(&self, user_id: String) -> Vec<Activity> {
        // Implementation for listing activities
        Vec::<Activity>::new()
    }
}
