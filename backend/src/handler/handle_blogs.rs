use axum::{
    extract::{Query, State},
    Json
};
use std::collections::HashMap;
use std::sync::Arc;

use super::handler::Handler;
use crate::service::service::{Service, BlogService};

impl Handler {
    pub async fn get_blogs(Query(params): Query<HashMap<String, String>>, state: State<Arc<Service>>) -> Json<serde_json::Value> {
        let year = params.get("year");
        let month = params.get("month");

        let service = state.0;

        service.clone().create_blog();

        Json(serde_json::json!({
            "status": "success",
            "data": {
                "id": params.get("id").unwrap_or(&"unknown".to_string())
            }
        }))
    }
}