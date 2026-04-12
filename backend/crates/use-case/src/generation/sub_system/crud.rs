use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, SubSystemRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::SubSystemEntity;
use std::marker::PhantomData;

pub struct SubSystemUseCase<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: SubSystemRepositoryTrait<Tx>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: SubSystemRepositoryTrait<Tx>>
    ErrorMapperTrait for SubSystemUseCase<Tx, U, F, R>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: SubSystemRepositoryTrait<Tx>>
    SubSystemUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn create(self, input: SubSystemEntity) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.add(uow.ref_tx(), &input.into()).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }

    pub async fn get(self) -> Result<Vec<SubSystemEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let systems = self.repo.get(uow.ref_tx()).await?;
        Ok(systems.into_iter().map(|u| u.into()).collect())
    }
}
