use super::dto::LabelInOut;
use crate::interface::LabelRepositoryTrait;
use std::io::Error;

pub struct GetLabelsUseCase<R: LabelRepositoryTrait> {
    repo: R,
}

impl<R: LabelRepositoryTrait> GetLabelsUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get(self) -> Result<Vec<LabelInOut>, Error> {
        let labels = self.repo.get().await.map_err(Error::other)?;
        Ok(labels.into_iter().map(|u| u.into()).collect())
    }
}
