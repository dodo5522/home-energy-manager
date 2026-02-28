use super::get::Response as GetResponse;
use super::post::{HistoryPostRequest, HistoryPostResponse};
use crate::di::db::get_connection;
use crate::error_mapper::{map_bad_request, map_internal_server_error};
use crate::errors::ErrorResponse;
use axum::{Json, extract::Path, http::StatusCode};
use layer_infra_db::repository::history::HistoryRepository;
use layer_infra_db::unit_of_work::UnitOfWorkFactory;
use layer_use_case::history::{CreateHistoryUseCase, GetHistoryUseCase, HistoryInOut};

#[utoipa::path(
    post,
    tag = "Generation - History",
    description = "Create a new history record",
    path = "/generation/history",
    request_body = HistoryPostRequest,
    responses(
        (status = 201, description = "OK", body = HistoryPostResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn post_history(
    Json(body): Json<HistoryPostRequest>,
) -> Result<(StatusCode, Json<HistoryPostResponse>), (StatusCode, Json<ErrorResponse>)> {
    let energy = HistoryInOut {
        unit: body.unit.try_into().map_err(map_bad_request)?,
        sub_system: body.sub_system,
        label: body.label,
        value: body.value,
        monitored_at: body.monitored_at,
    };
    println!("Inserting history record: {:?}", energy);

    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = CreateHistoryUseCase::new(
        HistoryRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let created = use_case.create(energy).await;

    match created {
        Ok(history_id) => Ok((
            StatusCode::CREATED,
            Json(HistoryPostResponse { id: history_id }),
        )),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{error}"),
            }),
        )),
    }
}

#[utoipa::path(
    get,
    tag = "Generation - History",
    description = "Get a history record by id",
    path = "/generation/history/{id}",
    params(("id" = i64, Path, description = "User id")),
    responses(
        (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_history(
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<GetResponse>), (StatusCode, Json<ErrorResponse>)> {
    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = GetHistoryUseCase::new(
        HistoryRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let history = use_case.get(id).await.map_err(map_internal_server_error)?;

    match history {
        Some(history) => Ok((
            StatusCode::OK,
            Json(GetResponse {
                value: history.value,
                unit: history.unit.to_string(),
                sub_system: history.sub_system,
                label: history.label,
                monitored_at: history.monitored_at,
            }),
        )),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "History record not found".to_string(),
            }),
        )),
    }
}
