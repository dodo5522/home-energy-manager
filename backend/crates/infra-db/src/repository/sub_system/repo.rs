use crate::models::{groups::ActiveModel, prelude::Groups};
use layer_domain::entity;
use layer_use_case::interface::{GenerationError as Error, SubSystemRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct SubSystemRepository {
    db: DatabaseConnection,
}

impl SubSystemRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_db_err<E: std::fmt::Display>(e: E) -> Error {
        Error::DbError(format!("{e}"))
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
            .map_err(Self::map_db_err)?;

        Ok(res.last_insert_id)
    }

    async fn get(&self) -> Result<Vec<entity::SubSystemEntity>, Error> {
        let groups = Groups::find()
            .all(&self.db)
            .await
            .map_err(Self::map_db_err)?;

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

    async fn has(&self, system: &String) -> Result<bool, Error> {
        Err(Error::NotImplemented(
            "SubSystemRepository::has()".to_string(),
        ))
    }

    async fn delete(&self, system: &String) -> Result<(), Error> {
        Err(Error::NotImplemented(
            "SubSystemRepository::delete()".to_string(),
        ))
    }
}
