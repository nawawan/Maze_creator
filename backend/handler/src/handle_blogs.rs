use axum::{
    Json, extract::{Query, State}
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;

use crate::model::blog::BlogResponse;

use super::error::UsecaseError;
use super::handler::{Handler};
use super::model::blog::CreateBlogRequest;
use usecase::model::blog::BlogRequest;
use usecase::service::blog::blog_service::{BlogService};
use usecase::service::service::Service; 

impl Handler {
    pub async fn get_blogs(Query(params): Query<HashMap<String, String>>, state: State<Arc<Service>>) -> Json<serde_json::Value> {
        let year = params.get("year");
        let month = params.get("month");

        let service = state.0.clone();

        service.get_blogs(year, month);

        Json(serde_json::json!({
            "status": "success",
            "data": {
                "id": params.get("id").unwrap_or(&"unknown".to_string())
            }
        }))
    }

    pub async fn create_blog(Json(req): Json<CreateBlogRequest>, state: State<Arc<Service>>) -> Result<Json<BlogResponse>, UsecaseError> {
        let blog_req = BlogRequest {
            title: req.title,
            content: req.content,
        };

        let service = state.0.clone();

        let result = service.create_blog(blog_req).await;
        if let Err(ref e) = result {
            error!("Failed to create blog: {}", e.message);
        }
        let blog = result?;
        Ok(Json(blog.into()))
    }
}