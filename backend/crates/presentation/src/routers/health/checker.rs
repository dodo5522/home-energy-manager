use axum::http::StatusCode;

#[utoipa::path(
    get,
    tag = "Health",
    description = "Check the health of the service",
    path = "/health",
    responses(
        (status = 204, description = "the health check passed"),
    ),
)]
pub async fn check_health() -> StatusCode {
    StatusCode::NO_CONTENT
}
