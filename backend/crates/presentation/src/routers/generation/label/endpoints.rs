use super::{get::LabelItem, post::LabelPostRequest, update::UpdateLabelQuery};
use crate::di::db::get_connection;
use crate::error_mapper::{map_internal_server_error, map_not_found_error};
use crate::errors::ErrorResponse;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use layer_infra_db::repository::label::LabelRepository;
use layer_infra_db::unit_of_work::UnitOfWorkFactory;
use layer_use_case::label::{LabelInOut, LabelUseCase};

#[utoipa::path(
    post,
    tag = "Generation - Label",
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
    put,
    tag = "Generation - Label",
    description = "Update the specified label",
    path = "/generation/labels/{label}",
    params(
        UpdateLabelQuery,
        ("label", description = "label name"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn update_label(
    Path(label): Path<String>,
    Query(query): Query<UpdateLabelQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let label = LabelInOut {
        label: label,
        remark: query.remark,
    };
    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = LabelUseCase::new(
        LabelRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let _ = use_case.update(label).await.map_err(map_not_found_error)?;
    Ok(StatusCode::OK)
}

#[utoipa::path(
    get,
    tag = "Generation - Label",
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
    tag = "Generation - Label",
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
    let found = use_case
        .get(label.to_owned())
        .await
        .map_err(map_internal_server_error)?;

    if let Some(found_label) = found {
        Ok((StatusCode::OK, Json(found_label.into())))
    } else {
        Err(map_not_found_error(format!("Label '{}' not found", label)))
    }
}

#[utoipa::path(
    delete,
    tag = "Generation - Label",
    description = "Delete specified label",
    path = "/generation/labels/{label}",
    params(
        ("label", description = "label name"),
    ),
    responses(
        (status = 204, description = "OK", body = LabelItem),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_label(
    Path(label): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = LabelUseCase::new(
        LabelRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let _ = use_case.delete(label).await.map_err(map_not_found_error)?;
    Ok(StatusCode::NO_CONTENT)
}
