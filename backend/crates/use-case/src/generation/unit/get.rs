use super::dto::UnitInOut;
use crate::interface::UnitRepositoryTrait;
use layer_domain::value_object::Unit;
use std::io::Error;

pub struct GetUnitsUseCase<R: UnitRepositoryTrait> {
    repo: R,
}

impl<R: UnitRepositoryTrait> GetUnitsUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get(self) -> Result<Vec<UnitInOut>, Error> {
        let units = self.repo.get(None::<&Unit>).await.map_err(Error::other)?;
        Ok(units.into_iter().map(|u| u.into()).collect())
    }
}
