use super::dto::HistoryInOut;
use crate::interface::{HistoryRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait};
use std::io::Error;
use std::marker::PhantomData;

pub struct CreateHistoryUseCase<
    U: UnitOfWorkTrait,
    F: UnitOfWorkFactoryTrait<U>,
    R: HistoryRepositoryTrait,
> {
    repo: R,
    factory: F,
    _marker: PhantomData<U>,
}

impl<U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>, R: HistoryRepositoryTrait>
    CreateHistoryUseCase<U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker: PhantomData,
        }
    }

    pub async fn create(self, input: HistoryInOut) -> Result<i64, Error> {
        let uow = self.factory.begin().await?;
        let h = self.repo.add(&input.into()).await;
        match h {
            Ok(history) => {
                uow.commit().await?;
                Ok(history.into())
            }
            Err(e) => {
                uow.rollback().await?;
                Err(Error::other(e))
            }
        }
    }
}
