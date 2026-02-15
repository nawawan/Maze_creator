#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CreateBlogRequest {
    pub title: String,
    pub content: String,
}