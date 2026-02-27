use super::dto::LabelInOut;
use crate::interface::{LabelRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait};
use std::io::Error;
use std::marker::PhantomData;

pub struct LabelUseCase<U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>, R: LabelRepositoryTrait> {
    repo: R,
    factory: F,
    _marker: PhantomData<U>,
}

impl<U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>, R: LabelRepositoryTrait>
    LabelUseCase<U, F, R>
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

    pub async fn get(self) -> Result<Vec<LabelInOut>, Error> {
        let labels = self.repo.get(None).await.map_err(Error::other)?;
        Ok(labels.into_iter().map(|u| u.into()).collect())
    }

    pub async fn has(self, label: impl AsRef<str>) -> Result<bool, Error> {
        let labels = self
            .repo
            .get(Some(label.as_ref()))
            .await
            .map_err(Error::other)?;
        Ok(!labels.is_empty())
    }

    pub async fn update(self, input: LabelInOut) -> Result<(), Error> {
        let _ = self
            .repo
            .update(&input.into())
            .await
            .map_err(Error::other)?;
        Ok(())
    }

    pub async fn delete(self, label: impl AsRef<str>) -> Result<(), Error> {
        let _ = self
            .repo
            .delete(label.as_ref())
            .await
            .map_err(Error::other)?;
        Ok(())
    }
}
