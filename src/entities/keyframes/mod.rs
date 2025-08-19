#[derive(Debug, Clone, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::keyframes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KeyframeEntity {
    id: i64,
    video_related_frame_id: i16,
    video_related_frame_timestamp: f64,
}

impl KeyframeEntity {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn video_related_frame_id(&self) -> i16 {
        self.video_related_frame_id
    }

    pub fn video_related_frame_timestamp(&self) -> f64 {
        self.video_related_frame_timestamp
    }
}
