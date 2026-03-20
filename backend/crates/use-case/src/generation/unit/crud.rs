use crate::interface::{
    GenerationError, UnitOfWorkFactoryTrait, UnitOfWorkTrait, UnitRepositoryTrait,
};
use layer_domain::{
    entity::UnitEntity,
    value_object::{Unit, UnitError},
};
use std::{
    io::{Error, ErrorKind},
    marker::PhantomData,
};

pub struct UnitUseCase<R: UnitRepositoryTrait, U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>> {
    repo: R,
    factory: F,
    _marker: PhantomData<U>,
}

impl<U: UnitOfWorkTrait, F: UnitOfWorkFactoryTrait<U>, R: UnitRepositoryTrait>
    UnitUseCase<R, U, F>
{
    fn map_err_unit_value(e: UnitError) -> Error {
        match e {
            UnitError::Empty => Error::new(ErrorKind::InvalidInput, "unit must not be empty"),
            UnitError::Blank => Error::new(ErrorKind::InvalidInput, "unit must not be blank"),
            UnitError::Invalid(s) => {
                Error::new(ErrorKind::InvalidInput, format!("'{s}' is invalid"))
            }
        }
    }

    fn map_err_generation_error(e: GenerationError) -> Error {
        match e {
            GenerationError::DbError(s) => Error::new(ErrorKind::Other, s),
            GenerationError::Unknown(s) => Error::new(ErrorKind::Other, s),
            GenerationError::InvalidUnit(s) => Self::map_err_unit_value(UnitError::Invalid(s)),
            GenerationError::NotFound(s) => Error::new(ErrorKind::Other, s),
            _ => Error::new(ErrorKind::Other, format!("{e}")),
        }
    }

    pub fn new(repo: R, factory: F) -> Self {
        Self {
            repo,
            factory,
            _marker: PhantomData,
        }
    }

    pub async fn create(self, input: UnitEntity) -> Result<(), Error> {
        let uow = self.factory.begin().await?;
        if let Err(e) = self.repo.add(&input.into()).await {
            uow.rollback().await?;
            Err(Error::other(e))
        } else {
            uow.commit().await?;
            Ok(())
        }
    }

    pub async fn get(self, unit: impl AsRef<str>) -> Result<Option<UnitEntity>, Error> {
        let target: Unit = unit
            .as_ref()
            .to_string()
            .try_into()
            .map_err(Self::map_err_unit_value)?;
        let units = self
            .repo
            .get(Some(&target))
            .await
            .map_err(Self::map_err_generation_error)?;
        Ok(if let Some(unit) = units.first() {
            Some(unit.to_owned())
        } else {
            None
        })
    }

    pub async fn get_all(self) -> Result<Vec<UnitEntity>, Error> {
        let units = self
            .repo
            .get(None::<&Unit>)
            .await
            .map_err(Self::map_err_generation_error)?;
        Ok(units.into_iter().map(|u| u.into()).collect())
    }

    pub async fn update(self, input: UnitEntity) -> Result<(), Error> {
        let _ = self
            .repo
            .update(&input)
            .await
            .map_err(Self::map_err_generation_error)?;
        Ok(())
    }

    pub async fn delete(self, unit: impl AsRef<str>) -> Result<(), Error> {
        let _ = self
            .repo
            .delete(
                &unit
                    .as_ref()
                    .to_string()
                    .try_into()
                    .map_err(Self::map_err_unit_value)?,
            )
            .await
            .map_err(Self::map_err_generation_error)?;
        Ok(())
    }
}
