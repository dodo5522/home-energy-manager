use crate::models::{groups::ActiveModel, prelude::Groups};
use layer_domain::{entity, value_object};
use layer_use_case::interface::{GenerationRepositoryError as Error, GroupRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseConnection, entity::EntityTrait};

pub struct GroupRepository {
    db: DatabaseConnection,
}

impl GroupRepository {
    pub async fn new(db: DatabaseConnection) -> Result<Self, Error> {
        Ok(Self { db })
    }

    fn map_err_instance<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("instantiate group failed: {e}"))
    }

    fn map_err_insert<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("insert group failed: {e}"))
    }

    pub fn map_err_find<E: std::fmt::Display>(e: E) -> Error {
        Error::Infra(format!("find group failed: {e}"))
    }
}

#[async_trait::async_trait]
impl GroupRepositoryTrait for GroupRepository {
    async fn add(&self, new: &entity::GroupRecord) -> Result<value_object::SubSystem, Error> {
        let group = ActiveModel {
            group: ActiveValue::Set(new.sub_system.to_owned().into()),
            remark: ActiveValue::Set(new.remark.to_owned()),
            ..Default::default()
        };

        let res = Groups::insert(group)
            .exec(&self.db)
            .await
            .map_err(Self::map_err_insert)?;

        Ok(value_object::SubSystem::new(res.last_insert_id).map_err(Self::map_err_instance)?)
    }

    async fn get(&self) -> Result<Vec<entity::GroupRecord>, Error> {
        let groups = Groups::find()
            .all(&self.db)
            .await
            .map_err(Self::map_err_find)?;

        let records = groups
            .into_iter()
            .map(|g| {
                Ok(entity::GroupRecord {
                    sub_system: g.group.try_into().map_err(Self::map_err_find)?,
                    remark: g.remark,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(records)
    }

    async fn has(&self, label: &value_object::SubSystem) -> Result<bool, Error> {
        todo!()
    }

    async fn delete(&self, label: &value_object::SubSystem) -> Result<(), Error> {
        todo!()
    }
}
