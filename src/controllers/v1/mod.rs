use utoipa_axum::router::OpenApiRouter;

use crate::models::states::AppState;

mod keyframes;
mod videos;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/keyframes", keyframes::router())
        .nest("/videos", videos::router())
}
