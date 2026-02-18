use super::dto::UnitInOut;
use crate::interface::{UnitOfWorkFactoryTrait, UnitOfWorkTrait, UnitRepositoryTrait};
use std::io::Error;
use std::marker::PhantomData;

pub struct CreateUnitUseCase<
    R: UnitRepositoryTrait,
    U: UnitOfWorkTrait,
    F: UnitOfWorkFactoryTrait<U>,
> {
    repo: R,
    factory: F,
    _marker: PhantomData<U>,
}

impl<U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>, R: UnitRepositoryTrait>
    CreateUnitUseCase<R, U, F>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker: PhantomData,
        }
    }

    pub async fn create(self, input: UnitInOut) -> Result<(), Error> {
        let uow = self.factory.begin().await?;
        if let Err(e) = self.repo.add(&input.into()).await {
            uow.rollback().await?;
            Err(Error::other(e))
        } else {
            uow.commit().await?;
            Ok(())
        }
    }
}
