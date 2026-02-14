use crate::di::db::get_connection;
use crate::dto::{
    errors::ErrorResponse,
    generation::history::get::Response as GetResponse,
    generation::history::post::{Request as PostRequest, Response as PostResponse},
};
use axum::{Json, extract::Path, http::StatusCode};
use layer_infra_db::repository::history::GenerationRepository;
use layer_infra_db::unit_of_work::UnitOfWorkFactory;
use layer_use_case::create_history::{CreateHistoryInput, CreateHistoryUseCase};

#[utoipa::path(
    post,
    tag = "Generation",
    description = "Create a new history record",
    path = "/generation/history",
    request_body = PostRequest,
    responses(
        (status = 201, description = "OK", body = PostResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn post_history(
    Json(body): Json<PostRequest>,
) -> Result<(StatusCode, Json<PostResponse>), (StatusCode, Json<ErrorResponse>)> {
    let energy = CreateHistoryInput {
        unit: body.unit.try_into().map_err(map_bad_request)?,
        sub_system: body.sub_system,
        label: body.label,
        value: body.value,
        monitored_at: body.monitored_at,
    };
    println!("Inserting test record: {:?}", energy);

    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = CreateHistoryUseCase::new(
        GenerationRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );

    if let Ok(history_id) = use_case.create(energy).await {
        Ok((StatusCode::CREATED, Json(PostResponse { id: history_id })))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Create history failed".to_string(),
            }),
        ))
    }
}

#[utoipa::path(
    get,
    tag = "Generation",
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
    // TODO: Implement actual fetching logic
    Ok((
        StatusCode::OK,
        Json(GetResponse {
            id,
            value: 123.4,
            unit: "kWh".into(),
            sub_system: "Battery".into(),
            energy_source: "Solar".into(),
            label: "Sample Label".into(),
            monitored_at: chrono::Utc::now(),
        }),
    ))
}

fn map_bad_request<E: std::fmt::Display>(e: E) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            message: format!("{e}"),
        }),
    )
}

fn map_internal_server_error<E: std::fmt::Display>(e: E) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            message: format!("{e}"),
        }),
    )
}
