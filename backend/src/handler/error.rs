use axum::{http::StatusCode, Json};
trait ErrorHandler {
    fn to_response(&self) -> (StatusCode, Json<serde_json::Value>);
}