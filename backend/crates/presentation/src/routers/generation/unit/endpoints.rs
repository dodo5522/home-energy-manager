use super::{get::UnitItem, post::UnitPostRequest, put::UpdateUnitQuery};
use crate::{connectors::db, error_mapper::ErrorMapperTrait, errors::ErrorResponse};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use layer_domain::entity::UnitEntity;
use layer_infra_db::{repository::unit::UnitRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::interface::GenerationError;
use layer_use_case::unit::UnitUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

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
        unit: body
            .unit
            .try_into()
            .map_err(ErrorMapper::map_to_bad_request)?,
        remark: body.remark,
    };
    println!("Inserting unit record: {:?}", unit);

    let use_case = UnitUseCase::new(
        UnitRepository {},
        UnitOfWorkFactory::new(
            db::get()
                .await
                .map_err(ErrorMapper::map_to_internal_server_error)?,
        ),
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
    let use_case = UnitUseCase::new(
        UnitRepository {},
        UnitOfWorkFactory::new(
            db::get()
                .await
                .map_err(ErrorMapper::map_to_internal_server_error)?,
        ),
    );
    let _ = use_case
        .update(UnitEntity {
            unit: unit.try_into().map_err(ErrorMapper::map_to_bad_request)?,
            remark: query.remark,
        })
        .await
        .map_err(ErrorMapper::map_generation_error)?;
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
    let use_case = UnitUseCase::new(
        UnitRepository {},
        UnitOfWorkFactory::new(
            db::get()
                .await
                .map_err(ErrorMapper::map_to_internal_server_error)?,
        ),
    );
    let units = use_case
        .get_all()
        .await
        .map_err(ErrorMapper::map_generation_error)?;

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
    let use_case = UnitUseCase::new(
        UnitRepository {},
        UnitOfWorkFactory::new(
            db::get()
                .await
                .map_err(ErrorMapper::map_to_internal_server_error)?,
        ),
    );
    let found = use_case
        .get(&unit)
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    if let Some(found_unit) = found {
        Ok((StatusCode::OK, Json(found_unit.into())))
    } else {
        Err(ErrorMapper::map_generation_error(
            GenerationError::NotFound(format!("Unit '{unit}' not found")),
        ))
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
    let use_case = UnitUseCase::new(
        UnitRepository {},
        UnitOfWorkFactory::new(
            db::get()
                .await
                .map_err(ErrorMapper::map_to_internal_server_error)?,
        ),
    );
    let _ = use_case
        .delete(&unit)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
