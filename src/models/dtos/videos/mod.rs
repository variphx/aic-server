use crate::models::entities::videos::VideoEntity;

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct VideoDto {
    id: i64,
    path: String,
}

impl From<VideoEntity> for VideoDto {
    fn from(value: VideoEntity) -> Self {
        let path = format!("/static/videos/L{}_V{:0>3}.mp4", value.l(), value.v());
        Self {
            id: value.id(),
            path,
        }
    }
}
