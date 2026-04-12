use crate::error_mapper::ErrorMapperTrait;
use crate::interface::{
    GenerationError, LabelRepositoryTrait, UnitOfWorkFactoryTrait, UnitOfWorkTrait,
};
use layer_domain::entity::LabelEntity;
use std::marker::PhantomData;

pub struct LabelUseCase<
    Tx,
    U: UnitOfWorkTrait<Tx>,
    F: UnitOfWorkFactoryTrait<Tx, U>,
    R: LabelRepositoryTrait<Tx>,
> {
    repo: R,
    factory: F,
    _marker0: PhantomData<Tx>,
    _marker1: PhantomData<U>,
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: LabelRepositoryTrait<Tx>>
    ErrorMapperTrait for LabelUseCase<Tx, U, F, R>
{
}

impl<Tx, U: UnitOfWorkTrait<Tx>, F: UnitOfWorkFactoryTrait<Tx, U>, R: LabelRepositoryTrait<Tx>>
    LabelUseCase<Tx, U, F, R>
{
    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker0: PhantomData,
            _marker1: PhantomData,
        }
    }

    pub async fn create(self, input: LabelEntity) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.add(uow.ref_tx(), &input.into()).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }

    pub async fn get(self, label: impl AsRef<str>) -> Result<Option<LabelEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let labels = self.repo.get(uow.ref_tx(), Some(label.as_ref())).await?;

        if let Some(label) = labels.first() {
            Ok(Some(label.to_owned().into()))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all(self) -> Result<Vec<LabelEntity>, GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        let labels = self.repo.get(uow.ref_tx(), None::<&str>).await?;
        Ok(labels.into_iter().map(|u| u.into()).collect())
    }

    pub async fn update(self, input: LabelEntity) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.update(uow.ref_tx(), &input.into()).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }

    pub async fn delete(self, label: impl AsRef<str>) -> Result<(), GenerationError> {
        let uow = self.factory.begin().await.map_err(Self::map_db_err)?;
        if let Err(e) = self.repo.delete(uow.ref_tx(), label.as_ref()).await {
            uow.rollback().await.map_err(Self::map_db_err)?;
            Err(e)
        } else {
            uow.commit().await.map_err(Self::map_db_err)?;
            Ok(())
        }
    }
}
