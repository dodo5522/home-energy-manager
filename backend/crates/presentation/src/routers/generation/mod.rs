use axum::Router;
use axum::routing::{get, post};

pub(crate) mod history;

pub fn route() -> Router {
    Router::new()
        .merge(Router::new().route("/history", post(history::post_history)))
        .merge(Router::new().route("/history/{id}", get(history::get_history)))
}
