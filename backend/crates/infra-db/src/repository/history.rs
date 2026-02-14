use crate::models::{histories::ActiveModel, prelude::Histories};
use layer_domain::entity;
use layer_use_case::interface::{GenerationRepositoryError as Error, HistoryRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct GenerationRepository {
    db: DatabaseConnection,
}

impl GenerationRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn map_err_instance<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("instantiate history failed: {e}"))
    }

    pub fn map_err_insert<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("insert history error: {e}"))
    }

    pub fn map_err_find<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("find history failed: {e}"))
    }
}

#[async_trait::async_trait]
impl HistoryRepositoryTrait for GenerationRepository {
    async fn add(&self, new: &entity::HistoryRecord) -> Result<entity::HistoryId, Error> {
        let history = ActiveModel {
            unit: ActiveValue::Set(new.unit.to_owned().into()),
            group: ActiveValue::Set(new.sub_system.to_owned().into()),
            label: ActiveValue::Set(new.label.to_owned()),
            source: ActiveValue::Set(new.energy_source.to_owned().into()),
            value: ActiveValue::Set(new.value.to_owned()),
            monitored_at: ActiveValue::Set(new.monitored_at.into()),
            ..Default::default()
        };

        let res = Histories::insert(history)
            .exec(&self.db)
            .await
            .map_err(Self::map_err_insert)?;

        Ok(entity::HistoryId(res.last_insert_id))
    }

    async fn get(&self, id: entity::HistoryId) -> Result<entity::HistoryRecord, Error> {
        let history = Histories::find_by_id::<i64>(id.into())
            .one(&self.db)
            .await
            .map_err(Self::map_err_find)?
            .ok_or(Error::Infra("not history".to_owned()))?;

        Ok(entity::HistoryRecord {
            id: Some(entity::HistoryId(history.id)),
            value: history.value,
            unit: history.unit.try_into().map_err(Self::map_err_find)?,
            sub_system: history.group.try_into().map_err(Self::map_err_find)?,
            energy_source: history.source.try_into().map_err(Self::map_err_find)?,
            label: history.label,
            monitored_at: history.monitored_at.into(),
        })
    }

    async fn delete(&self, id: entity::HistoryId) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }
}
