use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, HistoryRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::HistoryEntity;
use std::marker::PhantomData;

pub struct HistoryUseCase<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: HistoryRepositoryTrait<Tx>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: HistoryRepositoryTrait<Tx>>
    ErrorMapperTrait for HistoryUseCase<Tx, U, F, R>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: HistoryRepositoryTrait<Tx>>
    HistoryUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn create(self, input: HistoryEntity) -> Result<i64, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        match self.repo.add(uow.ref_tx(), &input.into()).await {
            Ok(history) => {
                uow.commit().await.map_err(Self::map_db_err)?;
                Ok(history.into())
            }
            Err(e) => {
                uow.rollback().await.map_err(Self::map_db_err)?;
                Err(e)
            }
        }
    }

    pub async fn get(self, id: i64) -> Result<Option<HistoryEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let history = self.repo.get(uow.ref_tx(), id.into()).await?;

        match history {
            Some(history) => Ok(Some(history.into())),
            None => Ok(None),
        }
    }
}
