use super::dto::SubSystemInOut;
use crate::interface::{SubSystemRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait};
use std::io::Error;
use std::marker::PhantomData;

pub struct CreateSubSystemUseCase<
    U: UnitOfWorkTrait,
    F: UnitOfWorkFactoryTrait<U>,
    R: SubSystemRepositoryTrait,
> {
    repo: R,
    factory: F,
    _marker: PhantomData<U>,
}

impl<U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>, R: SubSystemRepositoryTrait>
    CreateSubSystemUseCase<U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker: PhantomData,
        }
    }

    pub async fn create(self, input: SubSystemInOut) -> Result<(), Error> {
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
