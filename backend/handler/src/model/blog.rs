use usecase::model::blog::Blog;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CreateBlogRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UpdateBlogRequest {
    pub id: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: String,
    pub status: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetBlogRequest {
    pub id: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetBlogResponse {
    pub blog: BlogResponse,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BlogResponse {
    pub id: String,
    pub title: String,
    pub content_key: String,
    pub slug: String,
    pub status: String,
    pub published_at: Option<String>,
    pub tag: Option<String>,
}

impl From<Blog> for BlogResponse {
    fn from(blog: Blog) -> Self {
        Self {
            id: blog.id.to_string(),
            title: blog.title,
            content_key: blog.content_key,
            slug: blog.slug,
            status: blog.status.to_string(),
            published_at: blog.published_at.map(|dt| dt.and_utc().to_rfc3339()),
            tag: blog.tag,
        }
    }
}
