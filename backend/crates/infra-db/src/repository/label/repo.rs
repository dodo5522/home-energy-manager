use crate::{
    error_mapper::ErrorMapperTrait,
    models::{labels::ActiveModel, prelude::Labels},
};
use layer_domain::entity::LabelEntity;
use layer_use_case::interface::{GenerationError, LabelRepositoryTrait};
use sea_orm::{DatabaseTransaction, entity::EntityTrait};

pub struct LabelRepository {}

impl ErrorMapperTrait for LabelRepository {}

#[async_trait::async_trait]
impl LabelRepositoryTrait<DatabaseTransaction> for LabelRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        e: &LabelEntity,
    ) -> Result<String, GenerationError> {
        let res = Labels::insert::<ActiveModel>(e.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(res.last_insert_id)
    }

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        label: Option<impl AsRef<str> + Send>,
    ) -> Result<Vec<LabelEntity>, GenerationError> {
        if let Some(label) = label {
            let found = Labels::find_by_id(label.as_ref().to_string())
                .one(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if let Some(label) = found {
                Ok(vec![label.into()])
            } else {
                Ok(vec![])
            }
        } else {
            let labels = Labels::find()
                .all(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            let records = labels
                .into_iter()
                .map(|label| Ok(label.into()))
                .collect::<Result<_, _>>()?;
            Ok(records)
        }
    }

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        e: &LabelEntity,
    ) -> Result<LabelEntity, GenerationError> {
        let result = Labels::update::<ActiveModel>(e.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.into())
    }

    async fn delete(
        &self,
        tx: &DatabaseTransaction,
        label: impl AsRef<str> + Send,
    ) -> Result<(), GenerationError> {
        let result = Labels::delete_by_id(label.as_ref().to_string())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        if result.rows_affected == 1 {
            Ok(())
        } else {
            Err(GenerationError::NotFound(label.as_ref().to_string()))
        }
    }
}
