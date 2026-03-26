use crate::interface::{LabelRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait};
use layer_domain::entity::LabelEntity;
use std::{io::Error, marker::PhantomData};

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

    pub async fn create(self, input: LabelEntity) -> Result<(), Error> {
        let uow = self.factory.begin().await?;
        if let Err(e) = self.repo.add(&input.into()).await {
            uow.rollback().await?;
            Err(Error::other(e))
        } else {
            uow.commit().await?;
            Ok(())
        }
    }

    pub async fn get(self, label: impl AsRef<str>) -> Result<Option<LabelEntity>, Error> {
        let labels = self
            .repo
            .get(Some(label.as_ref()))
            .await
            .map_err(Error::other)?;

        if let Some(label) = labels.first() {
            Ok(Some(label.to_owned().into()))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all(self) -> Result<Vec<LabelEntity>, Error> {
        let labels = self.repo.get(None::<&str>).await.map_err(Error::other)?;
        Ok(labels.into_iter().map(|u| u.into()).collect())
    }

    pub async fn update(self, input: LabelEntity) -> Result<(), Error> {
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
