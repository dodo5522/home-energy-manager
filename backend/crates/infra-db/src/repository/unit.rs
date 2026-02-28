use crate::models::{prelude::Units, units::ActiveModel};
use layer_domain::{entity, value_object};
use layer_use_case::interface::{GenerationError as Error, UnitRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct UnitRepository {
    db: DatabaseConnection,
}

impl UnitRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_unknown_err<E: std::fmt::Display>(e: E) -> Error {
        Error::Unknown(format!("{e}"))
    }

    fn map_db_err<E: std::fmt::Display>(e: E) -> Error {
        Error::DbError(format!("{e}"))
    }

    fn map_invalid_unit(unit: String) -> Error {
        Error::InvalidUnit(unit)
    }
}

#[async_trait::async_trait]
impl UnitRepositoryTrait for UnitRepository {
    async fn add(&self, new: &entity::UnitEntity) -> Result<value_object::Unit, Error> {
        let unit = ActiveModel {
            unit: ActiveValue::Set(new.unit.to_owned().into()),
            remark: ActiveValue::Set(new.remark.to_owned()),
            ..Default::default()
        };

        let res = Units::insert(unit)
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;

        Ok(value_object::Unit::new(res.last_insert_id).map_err(Self::map_unknown_err)?)
    }

    async fn get(&self) -> Result<Vec<entity::UnitEntity>, Error> {
        let units = Units::find()
            .all(&self.db)
            .await
            .map_err(Self::map_db_err)?;

        let records = units
            .into_iter()
            .map(|u| {
                Ok(entity::UnitEntity {
                    unit: u
                        .unit
                        .clone()
                        .try_into()
                        .map_err(|_| Self::map_invalid_unit(u.unit))?,
                    remark: u.remark,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(records)
    }

    async fn has(&self, unit: &value_object::Unit) -> Result<bool, Error> {
        Err(Error::NotImplemented("UnitRepository::has()".to_string()))
    }

    async fn delete(&self, unit: &value_object::Unit) -> Result<(), Error> {
        Err(Error::NotImplemented(
            "UnitRepository::delete()".to_string(),
        ))
    }
}
