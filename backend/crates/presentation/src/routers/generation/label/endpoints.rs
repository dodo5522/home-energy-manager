use super::get::LabelItem;
use super::post::LabelPostRequest;
use crate::di::db::get_connection;
use crate::error_mapper::map_internal_server_error;
use crate::errors::ErrorResponse;
use axum::{Json, extract::Path, http::StatusCode};
use layer_infra_db::repository::label::LabelRepository;
use layer_infra_db::unit_of_work::UnitOfWorkFactory;
use layer_use_case::label::{LabelInOut, LabelUseCase};

#[utoipa::path(
    post,
    tag = "Generation",
    description = "Create a new label",
    path = "/generation/labels",
    request_body = LabelPostRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_label(
    Json(body): Json<LabelPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let label = LabelInOut {
        label: body.label,
        remark: body.remark,
    };
    println!("Inserting label record: {:?}", label);

    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = LabelUseCase::new(
        LabelRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );

    if let Err(e) = use_case.create(label).await {
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
    tag = "Generation",
    description = "Get existing labels",
    path = "/generation/labels",
    responses(
        (status = 200, description = "OK", body = Vec<LabelItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_labels()
-> Result<(StatusCode, Json<Vec<LabelItem>>), (StatusCode, Json<ErrorResponse>)> {
    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = LabelUseCase::new(
        LabelRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let labels = use_case
        .get_all()
        .await
        .map_err(map_internal_server_error)?;

    Ok((
        StatusCode::OK,
        Json(labels.into_iter().map(LabelItem::from).collect()),
    ))
}

#[utoipa::path(
    get,
    tag = "Generation",
    description = "Get specified label",
    path = "/generation/labels/{label}",
    params(
        ("label", description = "label name"),
    ),
    responses(
        (status = 200, description = "OK", body = LabelItem),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_label(
    Path(label): Path<String>,
) -> Result<(StatusCode, Json<LabelItem>), (StatusCode, Json<ErrorResponse>)> {
    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = LabelUseCase::new(
        LabelRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let label = use_case
        .get(label)
        .await
        .map_err(map_internal_server_error)?;

    Ok((StatusCode::OK, Json(label.into())))
}
