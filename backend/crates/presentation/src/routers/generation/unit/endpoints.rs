use super::get::UnitItem;
use super::{post::UnitPostRequest, put::UpdateUnitQuery};
use crate::connectors::db::get;
use crate::error_mapper::{map_bad_request, map_internal_server_error, map_not_found_error};
use crate::errors::ErrorResponse;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use layer_domain::entity::UnitEntity;
use layer_infra_db::repository::unit::UnitRepository;
use layer_infra_db::unit_of_work::UnitOfWorkFactory;
use layer_use_case::unit::UnitUseCase;
use sea_orm::Update;

#[utoipa::path(
    post,
    tag = "Generation - Unit",
    description = "Create a new unit",
    path = "/generation/units",
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
    let unit = UnitEntity {
        unit: body.unit.try_into().map_err(map_bad_request)?,
        remark: body.remark,
    };
    println!("Inserting unit record: {:?}", unit);

    let db = get().await.map_err(map_internal_server_error)?;
    let use_case = UnitUseCase::new(
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
    put,
    tag = "Generation - Unit",
    description = "Update the specified unit",
    path = "/generation/units/{unit}",
    params(
        UpdateUnitQuery,
        ("unit", description = "unit name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn update_unit(
    Path(unit): Path<String>,
    Query(query): Query<UpdateUnitQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let db = get().await.map_err(map_internal_server_error)?;
    let use_case = UnitUseCase::new(
        UnitRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let _ = use_case
        .update(UnitEntity {
            unit: unit.try_into().map_err(map_bad_request)?,
            remark: query.remark,
        })
        .await
        .map_err(map_not_found_error)?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    tag = "Generation - Unit",
    description = "Get existing units",
    path = "/generation/units",
    responses(
        (status = 200, description = "OK", body = Vec<UnitItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_units()
-> Result<(StatusCode, Json<Vec<UnitItem>>), (StatusCode, Json<ErrorResponse>)> {
    let db = get().await.map_err(map_internal_server_error)?;
    let use_case = UnitUseCase::new(
        UnitRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let units = use_case
        .get_all()
        .await
        .map_err(map_internal_server_error)?;

    Ok((
        StatusCode::OK,
        Json(units.into_iter().map(UnitItem::from).collect()),
    ))
}

#[utoipa::path(
    get,
    tag = "Generation - Unit",
    description = "Get existing unit",
    path = "/generation/units/{unit}",
    params(
        ("unit", description = "unit name"),
    ),
    responses(
        (status = 200, description = "OK", body = UnitItem),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_unit(
    Path(unit): Path<String>,
) -> Result<(StatusCode, Json<UnitItem>), (StatusCode, Json<ErrorResponse>)> {
    let db = get().await.map_err(map_internal_server_error)?;
    let use_case = UnitUseCase::new(
        UnitRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let found = use_case
        .get(&unit)
        .await
        .map_err(map_internal_server_error)?;

    if let Some(found_unit) = found {
        Ok((StatusCode::OK, Json(found_unit.into())))
    } else {
        Err(map_not_found_error(format!("Unit '{unit}' not found")))
    }
}

#[utoipa::path(
    delete,
    tag = "Generation - Unit",
    description = "Delete existing unit",
    path = "/generation/units/{unit}",
    params(
        ("unit", description = "unit name"),
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_unit(
    Path(unit): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let db = get().await.map_err(map_internal_server_error)?;
    let use_case = UnitUseCase::new(
        UnitRepository::new(db.clone()),
        UnitOfWorkFactory::new(db.clone()),
    );
    let _ = use_case
        .delete(&unit)
        .await
        .map_err(map_internal_server_error)?;
    Ok(StatusCode::NO_CONTENT)
}
