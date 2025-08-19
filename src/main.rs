use std::net::SocketAddr;

use axum::http::Method;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{docs::ApiDoc, states::AppState};

mod controllers;
mod docs;
mod dtos;
mod entities;
mod repositories;
mod schema;
mod services;
mod states;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", controllers::router())
        .split_for_parts();

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(AppState::new()?);

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 8080))).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
