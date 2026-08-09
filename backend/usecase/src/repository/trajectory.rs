use async_trait::async_trait;

use crate::errors::repo_error::RepoError;
use crate::model::activity::Activity;
use crate::model::trajectory::LodTrajectory;

#[async_trait]
pub trait TrajectoryRepository: Send + Sync {
    async fn create_trajectory(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        trajectory: LodTrajectory,
    ) -> Result<LodTrajectory, RepoError>;
    async fn upload_trajectory(
        &self,
        trajectory: LodTrajectory,
    ) -> Result<LodTrajectory, RepoError>;
    async fn get_trajectory(
        &self,
        activity_id: i64,
        zoom_level: i32,
    ) -> Result<LodTrajectory, RepoError>;

    async fn create_activity(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        activity: Activity,
    ) -> Result<Activity, RepoError>;
    async fn get_activity(&self, activity_id: String) -> Result<Activity, RepoError>;
    async fn list_activities(&self, user_id: String) -> Vec<Activity>;
}
