use super::dto::SubSystemInOut;
use crate::interface::SubSystemRepositoryTrait;
use std::io::Error;

pub struct GetSubSystemsUseCase<R: SubSystemRepositoryTrait> {
    repo: R,
}

impl<R: SubSystemRepositoryTrait> GetSubSystemsUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get(self) -> Result<Vec<SubSystemInOut>, Error> {
        let systems = self.repo.get().await.map_err(Error::other)?;
        Ok(systems.into_iter().map(|u| u.into()).collect())
    }
}
