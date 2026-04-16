use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, UnitOfWorkFactoryTrait, UnitOfWorkTrait, UnitRepositoryTrait,
};
use layer_domain::{entity::UnitEntity, value_object::Unit};
use std::marker::PhantomData;

pub struct UnitUseCase<
    Tx,
    R: UnitRepositoryTrait<Tx>,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: UnitRepositoryTrait<Tx>>
    ErrorMapperTrait for UnitUseCase<Tx, R, U, F>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: UnitRepositoryTrait<Tx>>
    UnitUseCase<Tx, R, U, F>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn create(self, input: UnitEntity) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.add(uow.ref_tx(), &input.into()).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }

    pub async fn get(self, unit: impl AsRef<str>) -> Result<Option<UnitEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let target: Unit = unit
            .as_ref()
            .to_string()
            .try_into()
            .map_err(Self::map_err_unit_value)?;
        let units = self.repo.get(uow.ref_tx(), Some(&target)).await?;

        Ok(if let Some(unit) = units.first() {
            Some(unit.to_owned())
        } else {
            None
        })
    }

    pub async fn get_all(self) -> Result<Vec<UnitEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let units = self.repo.get(uow.ref_tx(), None::<&Unit>).await?;
        Ok(units.into_iter().map(|u| u.into()).collect())
    }

    pub async fn update(self, input: UnitEntity) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;

        if let Err(e) = self.repo.update(uow.ref_tx(), &input).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }

    pub async fn delete(self, unit: impl AsRef<str>) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let unit: Unit = unit
            .as_ref()
            .to_string()
            .try_into()
            .map_err(Self::map_err_unit_value)?;

        if let Err(e) = self.repo.delete(uow.ref_tx(), &unit).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }
}
