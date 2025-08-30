use crate::models::entities::{keyframes::KeyframeEntity, videos::VideoEntity};

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct KeyframeDto {
    id: i64,
    path: String,
    frame_index: i64,
    frame_timestamp: f32,
}

impl From<(KeyframeEntity, VideoEntity)> for KeyframeDto {
    fn from((keyframe, video): (KeyframeEntity, VideoEntity)) -> Self {
        let id = keyframe.id();
        let frame_index = keyframe.frame_index();
        let frame_timestamp = keyframe.frame_timestamp();
        let video_id = video.id();
        let path = format!("/static/keyframes/{}/{:0>3}.jpg", video_id, keyframe.name());
        Self {
            id,
            path,
            frame_index,
            frame_timestamp,
        }
    }
}

impl From<(VideoEntity, KeyframeEntity)> for KeyframeDto {
    fn from((video, keyframe): (VideoEntity, KeyframeEntity)) -> Self {
        Self::from((keyframe, video))
    }
}
