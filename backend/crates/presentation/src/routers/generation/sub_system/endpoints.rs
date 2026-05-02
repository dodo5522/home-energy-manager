use super::{get::SubSystemItem, post::SubSystemPostRequest};
use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{Json, extract::State, http::StatusCode};
use layer_domain::entity::SubSystemEntity;
use layer_infra_db::{
    repository::sub_system::SubSystemRepository, unit_of_work::UnitOfWorkFactory,
};
use layer_use_case::sub_system::SubSystemUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    post,
    tag = "Generation - Sub System",
    description = "Create a new sub system",
    path = "/generation/sub_system",
    request_body = SubSystemPostRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_sub_system(
    State(state): State<RouterState>,
    Json(body): Json<SubSystemPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let system = SubSystemEntity {
        sub_system: body.sub_system,
        remark: body.remark,
    };
    println!("Inserting sub system record: {:?}", system);

    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = SubSystemUseCase::new(repo, factory);

    if let Err(e) = use_case.create(system).await {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{e}"),
            }),
        ))
    } else {
        Ok(StatusCode::CREATED)
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get existing sub systems",
    path = "/generation/sub_system",
    responses(
        (status = 200, description = "OK", body = Vec<SubSystemItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_sub_systems(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Json<Vec<SubSystemItem>>), (StatusCode, Json<ErrorResponse>)> {
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = SubSystemUseCase::new(repo, factory);
    let systems = use_case
        .get()
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    Ok((
        StatusCode::OK,
        Json(systems.into_iter().map(SubSystemItem::from).collect()),
    ))
}
