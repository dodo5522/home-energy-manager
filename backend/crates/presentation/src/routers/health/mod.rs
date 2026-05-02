use crate::routers::RouterState;
use axum::{Router, routing::get};
pub(crate) mod checker;

pub fn route() -> Router<RouterState> {
    Router::<RouterState>::new().route("/", get(checker::check_health))
}
