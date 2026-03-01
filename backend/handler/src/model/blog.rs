use usecase::model::blog::{Blog, BlogStatus};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CreateBlogRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BlogResponse {
    pub id: String,
    pub title: String,
    pub content_key: String,
    pub status: String,
}

impl From<Blog> for BlogResponse {
    fn from(blog: Blog) -> Self {
        Self {
            id: blog.id.to_string(),
            title: blog.title,
            content_key: blog.content_key,
            status: blog.status.to_string(),
        }
    }
}