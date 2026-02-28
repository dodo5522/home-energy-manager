use super::get::SubSystemItem;
use super::post::SubSystemPostRequest;
use crate::di::db::get_connection;
use crate::error_mapper::map_internal_server_error;
use crate::errors::ErrorResponse;
use axum::{Json, http::StatusCode};
use layer_infra_db::repository::sub_system::SubSystemRepository;
use layer_infra_db::unit_of_work::UnitOfWorkFactory;
use layer_use_case::sub_system::{CreateSubSystemUseCase, GetSubSystemsUseCase, SubSystemInOut};

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
    Json(body): Json<SubSystemPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let system = SubSystemInOut {
        sub_system: body.sub_system,
        remark: body.remark,
    };
    println!("Inserting sub system record: {:?}", system);

    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = CreateSubSystemUseCase::new(
        SubSystemRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );

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
pub async fn get_sub_systems()
-> Result<(StatusCode, Json<Vec<SubSystemItem>>), (StatusCode, Json<ErrorResponse>)> {
    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = GetSubSystemsUseCase::new(SubSystemRepository::new(db.clone()));
    let systems = use_case.get().await.map_err(map_internal_server_error)?;

    Ok((
        StatusCode::OK,
        Json(systems.into_iter().map(SubSystemItem::from).collect()),
    ))
}
