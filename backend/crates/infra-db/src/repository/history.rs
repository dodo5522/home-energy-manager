use crate::models::{histories::ActiveModel, prelude::Histories};
use layer_domain::entity;
use layer_use_case::interface::{GenerationError as Error, HistoryRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct HistoryRepository {
    db: DatabaseConnection,
}

impl HistoryRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_db_err<E: std::fmt::Display>(e: E) -> Error {
        Error::DbError(format!("{e}"))
    }

    fn map_invalid_unit(unit: String) -> Error {
        Error::InvalidUnit(unit)
    }
}

#[async_trait::async_trait]
impl HistoryRepositoryTrait for HistoryRepository {
    async fn add(&self, new: &entity::HistoryEntity) -> Result<entity::HistoryId, Error> {
        let history = ActiveModel {
            unit: ActiveValue::Set(new.unit.to_owned().into()),
            group: ActiveValue::Set(new.sub_system.to_owned()),
            label: ActiveValue::Set(new.label.to_owned()),
            value: ActiveValue::Set(new.value.to_owned()),
            monitored_at: ActiveValue::Set(new.monitored_at.into()),
            ..Default::default()
        };

        let res = Histories::insert(history)
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;

        Ok(entity::HistoryId(res.last_insert_id))
    }

    async fn get(&self, id: entity::HistoryId) -> Result<Option<entity::HistoryEntity>, Error> {
        let h = Histories::find_by_id::<i64>(id.into())
            .one(&self.db)
            .await
            .map_err(Self::map_db_err)?;

        if let Some(history) = h {
            Ok(Some(entity::HistoryEntity {
                value: history.value,
                unit: history
                    .unit
                    .clone()
                    .try_into()
                    .map_err(|_| Self::map_invalid_unit(history.unit))?,
                sub_system: history.group,
                label: history.label,
                monitored_at: history.monitored_at.into(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: entity::HistoryId) -> Result<(), Error> {
        Err(Error::NotImplemented(
            "HistoryRepository::delete()".to_string(),
        ))
    }
}
