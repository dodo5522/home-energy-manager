use axum::Router;
use axum::routing::{delete, get, post};

pub(crate) mod history;
pub(crate) mod label;
pub(crate) mod sub_system;
pub(crate) mod unit;

pub fn route() -> Router {
    Router::new()
        .merge(Router::new().route("/history", post(history::post_history)))
        .merge(Router::new().route("/history/{id}", get(history::get_history)))
        .merge(Router::new().route("/labels", post(label::post_label).get(label::get_labels)))
        .merge(Router::new().route(
            "/labels/{label}",
            delete(label::delete_label).get(label::get_label),
        ))
        .merge(Router::new().route(
            "/sub_system",
            post(sub_system::post_sub_system).get(sub_system::get_sub_systems),
        ))
        .merge(Router::new().route("/unit", post(unit::post_unit).get(unit::get_units)))
}
