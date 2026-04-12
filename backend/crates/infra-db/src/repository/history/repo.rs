use crate::models::{histories::ActiveModel, prelude::Histories};
use layer_domain::entity;
use layer_use_case::interface::{GenerationError, HistoryRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseTransaction, entity::EntityTrait};

pub struct HistoryRepository {}

impl HistoryRepository {
    fn map_db_err<E: std::fmt::Display>(e: E) -> GenerationError {
        GenerationError::DbError(format!("{e}"))
    }

    fn map_invalid_unit(unit: String) -> GenerationError {
        GenerationError::InvalidUnit(unit)
    }
}

#[async_trait::async_trait]
impl HistoryRepositoryTrait<DatabaseTransaction> for HistoryRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        new: &entity::HistoryEntity,
    ) -> Result<i64, GenerationError> {
        let history = ActiveModel {
            unit: ActiveValue::Set(new.unit.to_owned().into()),
            group: ActiveValue::Set(new.sub_system.to_owned()),
            label: ActiveValue::Set(new.label.to_owned()),
            value: ActiveValue::Set(new.value.to_owned()),
            monitored_at: ActiveValue::Set(new.monitored_at.into()),
            ..Default::default()
        };

        let res = Histories::insert(history)
            .exec(tx)
            .await
            .map_err(Self::map_db_err)?;

        Ok(res.last_insert_id)
    }

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        id: i64,
    ) -> Result<Option<entity::HistoryEntity>, GenerationError> {
        let h = Histories::find_by_id::<i64>(id.into())
            .one(tx)
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

    async fn delete(&self, tx: &DatabaseTransaction, id: i64) -> Result<(), GenerationError> {
        Err(GenerationError::NotImplemented(
            "HistoryRepository::delete()".to_string(),
        ))
    }
}
