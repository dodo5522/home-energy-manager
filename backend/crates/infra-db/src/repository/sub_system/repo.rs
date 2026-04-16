use crate::{
    error_mapper::ErrorMapperTrait,
    models::{groups::ActiveModel, prelude::Groups},
};
use layer_domain::entity;
use layer_use_case::interface::{GenerationError, SubSystemRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseTransaction, entity::EntityTrait};

pub struct SubSystemRepository {}

impl ErrorMapperTrait for SubSystemRepository {}

#[async_trait::async_trait]
impl SubSystemRepositoryTrait<DatabaseTransaction> for SubSystemRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        new: &entity::SubSystemEntity,
    ) -> Result<String, GenerationError> {
        let group = ActiveModel {
            group: ActiveValue::Set(new.sub_system.to_owned()),
            remark: ActiveValue::Set(new.remark.to_owned()),
            ..Default::default()
        };

        let res = Groups::insert(group)
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        Ok(res.last_insert_id)
    }

    async fn get(
        &self,
        tx: &DatabaseTransaction,
    ) -> Result<Vec<entity::SubSystemEntity>, GenerationError> {
        let groups = Groups::find()
            .all(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

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

    async fn has(
        &self,
        tx: &DatabaseTransaction,
        system: &String,
    ) -> Result<bool, GenerationError> {
        Err(GenerationError::NotImplemented(
            "SubSystemRepository::has()".to_string(),
        ))
    }

    async fn delete(
        &self,
        tx: &DatabaseTransaction,
        system: &String,
    ) -> Result<(), GenerationError> {
        Err(GenerationError::NotImplemented(
            "SubSystemRepository::delete()".to_string(),
        ))
    }
}
