use usecase::model::image::Image;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ImageResponse {
    pub id: String,
    pub url: String,
}


impl From<Image> for ImageResponse {
    fn from(image: Image) -> Self {
        ImageResponse {
            id: image.id,
            url: image.url,
        }
    }
}