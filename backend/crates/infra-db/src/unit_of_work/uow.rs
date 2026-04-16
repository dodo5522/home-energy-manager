use layer_use_case::interface::UnitOfWorkTrait;
use std::io::{Error, ErrorKind};

/// UnitOfWork struct. This has tx executor with I/F for TransactionExecutor.
pub struct UnitOfWork {
    tx: sea_orm::DatabaseTransaction,
}

/// Default UnitOfWork with SeaOrm transaction.
impl UnitOfWork {
    pub fn new(tx: sea_orm::DatabaseTransaction) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl UnitOfWorkTrait<sea_orm::DatabaseTransaction> for UnitOfWork {
    fn ref_tx(&self) -> &sea_orm::DatabaseTransaction {
        &self.tx
    }

    async fn commit(self) -> Result<(), Error> {
        Ok(self
            .tx
            .commit()
            .await
            .map_err(|e| Error::new(ErrorKind::ConnectionAborted, e))?)
    }

    async fn rollback(self) -> Result<(), Error> {
        Ok(self
            .tx
            .rollback()
            .await
            .map_err(|e| Error::new(ErrorKind::ConnectionAborted, e))?)
    }
}
