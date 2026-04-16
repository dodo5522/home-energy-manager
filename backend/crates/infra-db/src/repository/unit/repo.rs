use crate::{
    error_mapper::ErrorMapperTrait,
    models::{prelude::Units, units::ActiveModel},
};
use layer_domain::{entity::UnitEntity, value_object};
use layer_use_case::interface::{GenerationError, UnitRepositoryTrait};
use sea_orm::{DatabaseTransaction, DbErr, entity::EntityTrait};

pub struct UnitRepository {}

impl ErrorMapperTrait for UnitRepository {}

#[async_trait::async_trait]
impl UnitRepositoryTrait<DatabaseTransaction> for UnitRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        e: &UnitEntity,
    ) -> Result<value_object::Unit, GenerationError> {
        let res = Units::insert::<ActiveModel>(e.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(value_object::Unit::new(res.last_insert_id).map_err(Self::map_unknown_err)?)
    }

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        unit: Option<&value_object::Unit>,
    ) -> Result<Vec<UnitEntity>, GenerationError> {
        if let Some(unit) = unit {
            let unit = Units::find_by_id(unit.to_string())
                .one(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if let Some(unit) = unit {
                Ok(vec![unit.try_into()?])
            } else {
                Ok(vec![])
            }
        } else {
            let units = Units::find()
                .all(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            let records = units
                .into_iter()
                .map(|u| Ok(u.try_into()?))
                .collect::<Result<_, _>>()?;
            Ok(records)
        }
    }

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        e: &UnitEntity,
    ) -> Result<UnitEntity, GenerationError> {
        let result = Units::update::<ActiveModel>(e.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.try_into()?)
    }

    async fn delete(
        &self,
        tx: &DatabaseTransaction,
        unit: &value_object::Unit,
    ) -> Result<(), GenerationError> {
        let result = Units::delete_by_id::<String>(unit.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        if result.rows_affected > 0 {
            Ok(())
        } else {
            Err(GenerationError::NotFound(unit.into()))
        }
    }
}
