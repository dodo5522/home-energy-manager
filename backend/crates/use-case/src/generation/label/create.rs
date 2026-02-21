use super::dto::LabelInOut;
use crate::interface::{LabelRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait};
use std::io::Error;
use std::marker::PhantomData;

pub struct CreateLabelUseCase<
    U: UnitOfWorkTrait,
    F: UnitOfWorkFactoryTrait<U>,
    R: LabelRepositoryTrait,
> {
    repo: R,
    factory: F,
    _marker: PhantomData<U>,
}

impl<U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>, R: LabelRepositoryTrait>
    CreateLabelUseCase<U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker: PhantomData,
        }
    }

    pub async fn create(self, input: LabelInOut) -> Result<(), Error> {
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
