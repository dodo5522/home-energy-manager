use crate::models::{prelude::Sources, sources::ActiveModel};
use layer_domain::{entity, value_object};
use layer_use_case::interface::{GenerationRepositoryError as Error, SourceRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct SourceRepository {
    db: DatabaseConnection,
}

impl SourceRepository {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Error> {
        Ok(Self { db })
    }

    fn map_err_instance<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("instantiate group failed: {e}"))
    }

    fn map_err_insert<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("insert group failed: {e}"))
    }

    pub fn map_err_find<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("find group failed: {e}"))
    }
}

#[async_trait::async_trait]
impl SourceRepositoryTrait for SourceRepository {
    async fn add(&self, new: &entity::SourceRecord) -> Result<value_object::EnergySource, Error> {
        let source = ActiveModel {
            source: ActiveValue::Set(new.source.to_owned().into()),
            remark: ActiveValue::Set(new.remark.to_owned()),
            ..Default::default()
        };

        let res = Sources::insert(source)
            .exec(&self.db)
            .await
            .map_err(Self::map_err_insert)?;

        Ok(value_object::EnergySource::new(res.last_insert_id).map_err(Self::map_err_instance)?)
    }

    async fn get(&self) -> Result<Vec<entity::SourceRecord>, Error> {
        let sources = Sources::find()
            .all(&self.db)
            .await
            .map_err(Self::map_err_find)?;

        let records = sources
            .into_iter()
            .map(|s| {
                Ok(entity::SourceRecord {
                    source: s.source.try_into().map_err(Self::map_err_find)?,
                    remark: s.remark,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(records)
    }

    async fn has(&self, system: &value_object::EnergySource) -> Result<bool, Error> {
        todo!()
    }

    async fn delete(&self, system: &value_object::EnergySource) -> Result<(), Error> {
        todo!()
    }
}
