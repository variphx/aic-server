use utoipa::OpenApi;

pub const OPENAPI_TAG: &str = "AIC Server";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = OPENAPI_TAG, description = "AIC data server API")
    )
)]
pub struct ApiDoc;
