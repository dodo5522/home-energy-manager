use super::errors::ErrorResponse;
use axum::Json;
use axum::http::StatusCode;

pub(crate) fn map_bad_request<E: std::fmt::Display>(e: E) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            message: format!("{e}"),
        }),
    )
}

pub(crate) fn map_internal_server_error<E: std::fmt::Display>(
    e: E,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            message: format!("{e}"),
        }),
    )
}
