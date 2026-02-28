use axum::Router;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod consumption;
mod generation;
mod health;

pub fn route() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs/swagger").url("/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/docs/redoc", ApiDoc::openapi()))
        .nest("/health", health::route())
        .nest("/generation", generation::route())
}

#[derive(OpenApi)]
#[openapi(paths(
    health::checker::check_health,
    generation::history::get_history,
    generation::history::post_history,
    generation::label::delete_label,
    generation::label::post_label,
    generation::label::get_label,
    generation::label::get_labels,
    generation::label::update_label,
    generation::sub_system::post_sub_system,
    generation::sub_system::get_sub_systems,
    generation::unit::post_unit,
    generation::unit::get_units,
))]
pub(crate) struct ApiDoc {}
