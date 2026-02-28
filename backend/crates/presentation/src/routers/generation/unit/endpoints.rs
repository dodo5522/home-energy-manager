use super::get::UnitItem;
use super::post::UnitPostRequest;
use crate::di::db::get_connection;
use crate::error_mapper::{map_bad_request, map_internal_server_error};
use crate::errors::ErrorResponse;
use axum::{Json, http::StatusCode};
use layer_infra_db::repository::unit::UnitRepository;
use layer_infra_db::unit_of_work::UnitOfWorkFactory;
use layer_use_case::unit::{CreateUnitUseCase, GetUnitsUseCase, UnitInOut};

#[utoipa::path(
    post,
    tag = "Generation - Unit",
    description = "Create a new unit",
    path = "/generation/unit",
    request_body = UnitPostRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_unit(
    Json(body): Json<UnitPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let unit = UnitInOut {
        unit: body.unit.try_into().map_err(map_bad_request)?,
        remark: body.remark,
    };
    println!("Inserting unit record: {:?}", unit);

    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = CreateUnitUseCase::new(
        UnitRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );

    if let Err(e) = use_case.create(unit).await {
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
    tag = "Generation - Unit",
    description = "Get existing units",
    path = "/generation/unit",
    responses(
        (status = 200, description = "OK", body = Vec<UnitItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_units()
-> Result<(StatusCode, Json<Vec<UnitItem>>), (StatusCode, Json<ErrorResponse>)> {
    let db = get_connection().await.map_err(map_internal_server_error)?;
    let use_case = GetUnitsUseCase::new(UnitRepository::new(db.clone()));
    let units = use_case.get().await.map_err(map_internal_server_error)?;

    Ok((
        StatusCode::OK,
        Json(units.into_iter().map(UnitItem::from).collect()),
    ))
}
