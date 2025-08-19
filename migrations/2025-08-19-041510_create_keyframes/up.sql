CREATE TABLE keyframes (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES videos (id),
    video_related_frame_id SMALLINT NOT NULL,
    video_related_frame_timestamp DOUBLE PRECISION NOT NULL,
    UNIQUE (video_id, video_related_frame_id)
)
