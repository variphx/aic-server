// @generated automatically by Diesel CLI.

diesel::table! {
    keyframes (id) {
        id -> Int8,
        video_id -> Int8,
        video_related_frame_id -> Int2,
        video_related_frame_timestamp -> Float8,
    }
}

diesel::table! {
    videos (id) {
        id -> Int8,
        l -> Int2,
        v -> Int2,
    }
}

diesel::joinable!(keyframes -> videos (video_id));

diesel::allow_tables_to_appear_in_same_query!(keyframes, videos,);
