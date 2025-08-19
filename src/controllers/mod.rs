use utoipa_axum::router::OpenApiRouter;

use crate::states::AppState;

mod v1;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/v1", v1::router())
}
