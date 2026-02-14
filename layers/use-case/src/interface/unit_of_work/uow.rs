use std::io::Error;

#[async_trait::async_trait]
pub trait UnitOfWorkTrait: Send + Sync {
    /// Commit in the already begun transaction.
    async fn commit(self) -> Result<(), Error>;

    /// Rollback in the already begun transaction.
    async fn rollback(self) -> Result<(), Error>;
}
