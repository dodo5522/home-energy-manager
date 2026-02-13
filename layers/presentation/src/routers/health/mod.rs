use axum::{Router, routing::get};
pub(crate) mod checker;

pub fn route() -> Router {
    Router::new().route("/", get(checker::check_health))
}
