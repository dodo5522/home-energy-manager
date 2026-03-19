use crate::models::{labels::ActiveModel, prelude::Labels};
use layer_domain::entity::LabelEntity;
use layer_use_case::interface::{GenerationError, LabelRepositoryTrait};
use sea_orm::{DatabaseConnection, entity::EntityTrait};

pub struct LabelRepository {
    db: DatabaseConnection,
}

impl LabelRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_db_err<E: std::fmt::Display>(e: E) -> GenerationError {
        GenerationError::DbError(format!("{e}"))
    }
}

#[async_trait::async_trait]
impl LabelRepositoryTrait for LabelRepository {
    async fn add(&self, e: &LabelEntity) -> Result<String, GenerationError> {
        let res = Labels::insert::<ActiveModel>(e.into())
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;
        Ok(res.last_insert_id)
    }

    async fn get(
        &self,
        label: Option<impl AsRef<str> + Send>,
    ) -> Result<Vec<LabelEntity>, GenerationError> {
        if let Some(label) = label {
            let found = Labels::find_by_id(label.as_ref().to_string())
                .one(&self.db)
                .await
                .map_err(Self::map_db_err)?;
            if let Some(label) = found {
                Ok(vec![label.into()])
            } else {
                Ok(vec![])
            }
        } else {
            let labels = Labels::find()
                .all(&self.db)
                .await
                .map_err(Self::map_db_err)?;
            let records = labels
                .into_iter()
                .map(|label| Ok(label.into()))
                .collect::<Result<_, _>>()?;
            Ok(records)
        }
    }

    async fn update(&self, e: &LabelEntity) -> Result<LabelEntity, GenerationError> {
        let result = Labels::update::<ActiveModel>(e.into())
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;
        Ok(result.into())
    }

    async fn delete(&self, label: impl AsRef<str> + Send) -> Result<(), GenerationError> {
        let result = Labels::delete_by_id(label.as_ref().to_string())
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;
        if result.rows_affected == 1 {
            Ok(())
        } else {
            Err(GenerationError::NotFound(label.as_ref().to_string()))
        }
    }
}
