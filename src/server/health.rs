use std::sync::Arc;

use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::AppState;

#[derive(ToSchema, Serialize)]
struct HealthStatus {
    status: String,
    version: String,
}

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(health_check))
        .routes(routes!(health_details))
}

/// Basic health check
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, description = "Service is healthy", body = str)
    ),
    tag = "health"
)]
async fn health_check() -> &'static str {
    "ok"
}

/// Detailed health status
#[utoipa::path(
    get,
    path = "/details",
    responses(
        (status = OK, description = "Detailed health information", body = HealthStatus)
    ),
    tag = "health"
)]
async fn health_details() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: String::from("healthy"),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
