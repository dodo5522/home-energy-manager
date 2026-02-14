use super::dto::CreateHistoryInput;
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

    pub async fn create(self, input: CreateHistoryInput) -> Result<i64, Error> {
        let uow = self.factory.begin().await?;
        if let Ok(history) = self.repo.add(&input.into()).await {
            uow.commit().await?;
            Ok(history.into())
        } else {
            uow.rollback().await?;
            Err(Error::other("failed to create"))
        }
    }
}
