use axum::http::StatusCode;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 204, description = "the health check passed"),
    ),
)]
pub async fn check_health() -> StatusCode {
    StatusCode::NO_CONTENT
}
