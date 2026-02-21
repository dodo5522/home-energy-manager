use super::HistoryInOut;
use crate::interface::{HistoryRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait};
use std::io::Error;
use std::marker::PhantomData;

pub struct GetHistoryUseCase<
    R: HistoryRepositoryTrait,
    U: UnitOfWorkTrait,
    F: UnitOfWorkFactoryTrait<U>,
> {
    repo: R,
    factory: F,
    _marker: PhantomData<U>,
}

impl<R: HistoryRepositoryTrait, U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>>
    GetHistoryUseCase<R, U, F>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker: PhantomData,
        }
    }

    pub async fn get(self, id: i64) -> Result<Option<HistoryInOut>, Error> {
        let uow = self.factory.begin().await?;
        let history = self.repo.get(id.into()).await.map_err(Error::other)?;

        match history {
            Some(history) => {
                uow.commit().await?;
                Ok(Some(history.into()))
            }
            None => {
                uow.rollback().await?;
                Ok(None)
            }
        }
    }
}
