use super::UnitOfWorkTrait;
use std::io::Error;

#[async_trait::async_trait]
pub trait UnitOfWorkFactoryTrait<Tx, U: UnitOfWorkTrait<Tx>>: Send {
    /// begin transaction and return the unit of work instance.
    ///
    /// # Returns
    /// * Unit of work instance
    async fn begin(self) -> Result<U, Error>;
}
