use crate::di::db::get_connection;
use layer_domain::{entity, repository::IGenerationRepository};
use layer_infra_db::repository::generation::GenerationRepository;

use axum::http::StatusCode;
use axum::{Json, extract::Path};

use crate::dto::{
    errors::ErrorResponse,
    generation::history::get::Response as GetResponse,
    generation::history::post::{Request as PostRequest, Response as PostResponse},
};

#[utoipa::path(
    post,
    path = "/history",
    request_body = PostRequest,
    responses(
        (status = 201, description = "OK", body = PostResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn post_history(
    Json(body): Json<PostRequest>,
) -> Result<Json<PostResponse>, (StatusCode, Json<ErrorResponse>)> {
    let energy = entity::EnergyRecord {
        id: None,
        unit: body.unit.try_into().map_err(map_bad_request)?,
        sub_system: body.sub_system.try_into().map_err(map_bad_request)?,
        energy_source: body.energy_source.try_into().map_err(map_bad_request)?,
        label: body.label,
        value: body.value,
        monitored_at: body.monitored_at,
    };
    println!("Inserting test record: {:?}", energy);

    let db = get_connection().await.map_err(map_internal_server_error)?;
    let repo = GenerationRepository::new(db)
        .await
        .map_err(map_internal_server_error)?;

    if let Ok(res) = repo.add(&energy).await {
        let id = res.id.unwrap().0; // TODO: Handle unwrap safely
        Ok(Json(PostResponse { id }))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "".to_string(),
            }),
        ))
    }
}

#[utoipa::path(
    get,
    path = "/history/{id}",
    params(("id" = i64, Path, description = "User id")),
    responses(
        (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_history(
    Path(id): Path<i64>,
) -> Result<Json<GetResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement actual fetching logic
    Ok(Json(GetResponse {
        id,
        value: 123.4,
        unit: "kWh".into(),
        sub_system: "Battery".into(),
        energy_source: "Solar".into(),
        label: "Sample Label".into(),
        monitored_at: chrono::Utc::now(),
    }))
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
