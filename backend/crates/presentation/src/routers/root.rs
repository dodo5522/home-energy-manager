use crate::routers::{generation, health};
use axum::Router;
use http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

fn cors(allowed_origins: Vec<String>) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            allowed_origins
                .into_iter()
                .map(|origin| origin.parse::<HeaderValue>().expect("Invalid origin"))
                .collect::<Vec<HeaderValue>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}

pub fn route(allowed_origins: Vec<String>) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs/swagger").url("/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/docs/redoc", ApiDoc::openapi()))
        .nest("/health", health::route())
        .nest("/generation", generation::route())
        .layer(cors(allowed_origins))
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
    generation::unit::delete_unit,
    generation::unit::post_unit,
    generation::unit::get_unit,
    generation::unit::get_units,
    generation::unit::update_unit,
))]
pub(crate) struct ApiDoc {}
