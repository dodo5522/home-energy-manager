use crate::models::{groups::ActiveModel, prelude::Groups};
use layer_domain::entity;
use layer_use_case::interface::{GenerationRepositoryError as Error, SubSystemRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct SubSystemRepository {
    db: DatabaseConnection,
}

impl SubSystemRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_err_insert<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("insert group failed: {e}"))
    }

    fn map_err_find<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("find group failed: {e}"))
    }
}

#[async_trait::async_trait]
impl SubSystemRepositoryTrait for SubSystemRepository {
    async fn add(&self, new: &entity::SubSystemEntity) -> Result<String, Error> {
        let group = ActiveModel {
            group: ActiveValue::Set(new.sub_system.to_owned()),
            remark: ActiveValue::Set(new.remark.to_owned()),
            ..Default::default()
        };

        let res = Groups::insert(group)
            .exec(&self.db)
            .await
            .map_err(Self::map_err_insert)?;

        Ok(res.last_insert_id)
    }

    async fn get(&self) -> Result<Vec<entity::SubSystemEntity>, Error> {
        let groups = Groups::find()
            .all(&self.db)
            .await
            .map_err(Self::map_err_find)?;

        let records = groups
            .into_iter()
            .map(|g| {
                Ok(entity::SubSystemEntity {
                    sub_system: g.group,
                    remark: g.remark,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(records)
    }

    async fn has(&self, label: &String) -> Result<bool, Error> {
        todo!()
    }

    async fn delete(&self, label: &String) -> Result<(), Error> {
        todo!()
    }
}
