use crate::{app_state::AppState, logging::create_trace_layer, server::health, settings::Settings};
use thiserror::Error;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;

use super::api_doc::ApiDoc;

pub async fn serve(port: u16, app_settings: Settings) -> Result<(), ApiError> {
    let state = AppState::new(app_settings).await;

    let (router, _api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/health", health::router())
        // .layer(setup_request_tracing())
        .layer(create_trace_layer())
        .with_state(state)
        .split_for_parts();

    let router =
        router.merge(RapiDoc::with_openapi("/api-docs/openapi.json", _api).path("/rapidoc"));

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(ApiError::TcpListenerError)?;

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Axum server start error: {0}")]
    AxumError(#[from] std::io::Error),

    #[error("TCP listener bind error: {0}")]
    TcpListenerError(std::io::Error),
}
