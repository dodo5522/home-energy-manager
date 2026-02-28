use crate::models::{labels::ActiveModel, prelude::Labels};
use layer_domain::entity::LabelEntity;
use layer_use_case::interface::{GenerationError as Error, LabelRepositoryTrait};
use sea_orm::{DatabaseConnection, entity::EntityTrait};

pub struct LabelRepository {
    db: DatabaseConnection,
}

impl LabelRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_db_err<E: std::fmt::Display>(e: E) -> Error {
        Error::DbError(format!("{e}"))
    }
}

#[async_trait::async_trait]
impl LabelRepositoryTrait for LabelRepository {
    async fn add(&self, e: &LabelEntity) -> Result<String, Error> {
        let res = Labels::insert::<ActiveModel>(e.into())
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;
        Ok(res.last_insert_id)
    }

    async fn get(&self, label: Option<&str>) -> Result<Vec<LabelEntity>, Error> {
        if let Some(label) = label {
            let found = Labels::find_by_id(label)
                .one(&self.db)
                .await
                .map_err(Self::map_db_err)?;
            if let Some(model) = found {
                Ok(vec![LabelEntity {
                    label: model.label,
                    remark: Some(model.remark),
                }])
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
                .map(|model| {
                    Ok(LabelEntity {
                        label: model.label,
                        remark: Some(model.remark),
                    })
                })
                .collect::<Result<_, _>>()?;
            Ok(records)
        }
    }

    async fn update(&self, e: &LabelEntity) -> Result<LabelEntity, Error> {
        let result = Labels::update::<ActiveModel>(e.into())
            .validate()
            .map_err(Self::map_db_err)?
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;
        Ok(LabelEntity {
            label: result.label,
            remark: Some(result.remark),
        })
    }

    async fn delete(&self, label: &str) -> Result<(), Error> {
        let result = Labels::delete_by_id(label.to_owned())
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;
        if result.rows_affected == 1 {
            Ok(())
        } else {
            Err(Error::DbError(label.to_owned()))
        }
    }
}
