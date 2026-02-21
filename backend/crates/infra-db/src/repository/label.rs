use crate::models::{labels::ActiveModel, prelude::Labels};
use layer_domain::entity;
use layer_use_case::interface::{GenerationError as Error, LabelRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

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
    async fn add(&self, new: &entity::LabelEntity) -> Result<String, Error> {
        let label = ActiveModel {
            label: ActiveValue::Set(new.label.to_owned()),
            remark: ActiveValue::Set(new.remark.to_owned()),
            ..Default::default()
        };

        let res = Labels::insert(label)
            .exec(&self.db)
            .await
            .map_err(Self::map_db_err)?;

        Ok(res.last_insert_id)
    }

    async fn get(&self) -> Result<Vec<entity::LabelEntity>, Error> {
        let labels = Labels::find()
            .all(&self.db)
            .await
            .map_err(Self::map_db_err)?;

        let records = labels
            .into_iter()
            .map(|l| {
                Ok(entity::LabelEntity {
                    label: l.label,
                    remark: l.remark,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(records)
    }

    async fn has(&self, label: &str) -> Result<bool, Error> {
        Err(Error::NotImplemented("LabelRepository::has()".to_string()))
    }

    async fn delete(&self, label: &str) -> Result<(), Error> {
        Err(Error::NotImplemented(
            "LabelRepository::delete()".to_string(),
        ))
    }
}
