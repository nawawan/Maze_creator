use axum::{
    Json,
    extract::{Multipart, Path, Query, State},
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;

use crate::model::blog::BlogResponse;
use crate::model::image::ImageResponse;
use crate::{extractor::AuthorizedUser, model::blog::UpdateBlogRequest};

use super::error::UsecaseError;
use super::handler::Handler;
use usecase::model::blog::{BlogRequest, BlogStatus, BlogTag};
use usecase::service::blog::blog_service::BlogService;
use usecase::service::service::Service;

impl Handler {
    pub async fn list_blogs(
        Query(params): Query<HashMap<String, String>>,
        state: State<Arc<Service>>,
    ) -> Json<serde_json::Value> {
        let year = params.get("year");
        let month = params.get("month");
        let status = params.get("status").cloned().map(BlogStatus::from);
        let tag = params.get("tag");

        let service = state.0.clone();

        let blogs = service.list_blogs(year, month, status, tag).await;

        Json(serde_json::json!({
            "status": "success",
            "data": {
                "blogs": blogs.into_iter().map(BlogResponse::from).collect::<Vec<_>>()
            }
        }))
    }

    pub async fn get_blog(
        state: State<Arc<Service>>,
        Path(blog_id): Path<String>,
    ) -> Result<Json<serde_json::Value>, UsecaseError> {
        let service = state.0.clone();

        let blog = service.get_blog(blog_id).await?;
        Ok(Json(serde_json::json!({
            "status": "success",
            "data": {
                "blog": BlogResponse::from(blog)
            }
        })))
    }

    pub async fn create_draft(
        user: AuthorizedUser,
        state: State<Arc<Service>>,
    ) -> Result<Json<String>, UsecaseError> {
        let service = state.0.clone();

        if let Err(e) = validate_admin(&user) {
            error!("Permission denied: {}", e.error.message);
            return Err(e);
        }

        let id = service.create_draft().await?;

        Ok(Json(id))
    }

    pub async fn update_blog(
        user: AuthorizedUser,
        state: State<Arc<Service>>,
        Json(req): Json<UpdateBlogRequest>,
    ) -> Result<Json<BlogResponse>, UsecaseError> {
        if let Err(e) = validate_admin(&user) {
            error!("Permission denied: {}", e.error.message);
            return Err(e);
        }

        let tag = match req.tag {
            Some(t) => Some(
                BlogTag::try_from(t)
                    .map_err(|e| UsecaseError::bad_request(&e))?
                    .to_string(),
            ),
            None => None,
        };

        let blog_req = BlogRequest {
            id: req.id,
            title: req.title,
            slug: req.slug,
            content: req.content,
            status: req.status.map(BlogStatus::from),
            tag,
        };

        let service = state.0.clone();

        let result = service.update_blog(blog_req).await;
        if let Err(ref e) = result {
            error!("Failed to update blog: {}", e.message);
        }
        let blog = result?;
        Ok(Json(blog.into()))
    }

    pub async fn upload_blog_image(
        user: AuthorizedUser,
        state: State<Arc<Service>>,
        mut multipart: Multipart,
    ) -> Result<Json<ImageResponse>, UsecaseError> {
        if let Err(e) = validate_admin(&user) {
            error!("Permission denied: {}", e.error.message);
            return Err(e);
        }

        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| UsecaseError::bad_request(&e.body_text()))?
        {
            let name = field.name().unwrap_or("unknown").to_string();
            let data = field
                .bytes()
                .await
                .map_err(|e| UsecaseError::bad_request(&e.body_text()))?;
            if name != "image" {
                continue;
            }
            let service = state.0.clone();
            return service
                .upload_blog_image(data)
                .await
                .map(|image| Json(image.into()))
                .map_err(UsecaseError::from);
        }
        Err(UsecaseError::bad_request("No image field in multipart"))
    }
}

fn validate_admin(user: &AuthorizedUser) -> Result<(), UsecaseError> {
    if user.user.role == "admin" {
        Ok(())
    } else {
        Err(UsecaseError::permission_denied(
            "User does not have admin role",
        ))
    }
}
