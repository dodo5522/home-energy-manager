use std::io::Error;

#[async_trait::async_trait]
pub trait UnitOfWorkTrait<Tx>: Send {
    /// Get transaction.
    fn ref_tx(&self) -> &Tx;

    /// Commit in the already begun transaction.
    async fn commit(self) -> Result<(), Error>;

    /// Rollback in the already begun transaction.
    async fn rollback(self) -> Result<(), Error>;
}
