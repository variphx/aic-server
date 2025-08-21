use crate::models::entities::{keyframes::KeyframeEntity, videos::VideoEntity};

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct KeyframeDto {
    id: i64,
    path: String,
    timestamp: f64,
}

impl From<(KeyframeEntity, VideoEntity)> for KeyframeDto {
    fn from((keyframe, video): (KeyframeEntity, VideoEntity)) -> Self {
        let id = keyframe.id();
        let timestamp = keyframe.video_related_frame_timestamp();
        let path = format!(
            "/static/keyframes/L{}_V{:0>3}/{:0>3}.jpg",
            video.l(),
            video.v(),
            keyframe.video_related_frame_id()
        );
        Self {
            id,
            path,
            timestamp,
        }
    }
}

impl From<(VideoEntity, KeyframeEntity)> for KeyframeDto {
    fn from((video, keyframe): (VideoEntity, KeyframeEntity)) -> Self {
        Self::from((keyframe, video))
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct VectoredKeyframeDto {
    id: i64,
    score: f64,
}
