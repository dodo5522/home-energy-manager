use layer_use_case::interface::UnitOfWorkTrait;
use sea_orm::DatabaseTransaction;
use std::io::{Error, ErrorKind};

pub struct UnitOfWork {
    tx: DatabaseTransaction,
}

impl UnitOfWork {
    pub fn new(tx: DatabaseTransaction) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl UnitOfWorkTrait for UnitOfWork {
    async fn commit(self) -> Result<(), Error> {
        self.tx
            .commit()
            .await
            .map_err(|e| Error::new(ErrorKind::ConnectionAborted, e))?;
        Ok(())
    }

    async fn rollback(self) -> Result<(), std::io::Error> {
        self.tx
            .rollback()
            .await
            .map_err(|e| Error::new(ErrorKind::ConnectionAborted, e))?;
        Ok(())
    }
}
