use crate::models::entities::videos::VideoEntity;

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct VideoDto {
    id: i64,
    watch_url: String,
}

impl From<VideoEntity> for VideoDto {
    fn from(mut value: VideoEntity) -> Self {
        Self {
            id: value.id(),
            watch_url: std::mem::take(value.watch_url_mut()),
        }
    }
}
