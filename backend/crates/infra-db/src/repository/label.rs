use crate::models::{labels::ActiveModel, prelude::Labels};
use layer_domain::entity;
use layer_use_case::interface::{GenerationRepositoryError as Error, LabelRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct LabelRepository {
    db: DatabaseConnection,
}

impl LabelRepository {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Error> {
        Ok(Self { db })
    }

    fn map_err_insert<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("insert history error: {e}"))
    }

    fn map_err_find<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("find history failed: {e}"))
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
            .map_err(Self::map_err_insert)?;

        Ok(res.last_insert_id)
    }

    async fn get(&self) -> Result<Vec<entity::LabelEntity>, Error> {
        let labels = Labels::find()
            .all(&self.db)
            .await
            .map_err(Self::map_err_find)?;

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
        todo!()
    }

    async fn delete(&self, label: &str) -> Result<(), Error> {
        todo!()
    }
}
