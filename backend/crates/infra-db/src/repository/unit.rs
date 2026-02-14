use crate::models::{prelude::Units, units::ActiveModel};
use layer_domain::{entity, value_object};
use layer_use_case::interface::{GenerationRepositoryError as Error, UnitRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct UnitRepository {
    db: DatabaseConnection,
}

impl UnitRepository {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Error> {
        Ok(Self { db })
    }

    pub fn map_err_instance<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("instantiate unit failed: {e}"))
    }

    pub fn map_err_insert<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("insert unit error: {e}"))
    }

    pub fn map_err_find<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("find unit failed: {e}"))
    }
}

#[async_trait::async_trait]
impl UnitRepositoryTrait for UnitRepository {
    async fn add(&self, new: &entity::UnitRecord) -> Result<value_object::Unit, Error> {
        let unit = ActiveModel {
            unit: ActiveValue::Set(new.unit.to_owned().into()),
            remark: ActiveValue::Set(new.remark.to_owned()),
            ..Default::default()
        };

        let res = Units::insert(unit)
            .exec(&self.db)
            .await
            .map_err(Self::map_err_insert)?;

        Ok(value_object::Unit::new(res.last_insert_id).map_err(Self::map_err_instance)?)
    }

    async fn get(&self) -> Result<Vec<entity::UnitRecord>, Error> {
        let units = Units::find()
            .all(&self.db)
            .await
            .map_err(Self::map_err_find)?;

        let records = units
            .into_iter()
            .map(|u| {
                Ok(entity::UnitRecord {
                    unit: u.unit.try_into().map_err(Self::map_err_find)?,
                    remark: u.remark,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(records)
    }

    async fn has(&self, system: &value_object::Unit) -> Result<bool, Error> {
        todo!()
    }

    async fn delete(&self, system: &value_object::Unit) -> Result<(), Error> {
        todo!()
    }
}
