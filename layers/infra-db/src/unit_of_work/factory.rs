use super::UnitOfWork;
use layer_use_case::interface::UnitOfWorkFactoryTrait;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use std::io::{Error, ErrorKind};

pub struct UnitOfWorkFactory {
    db: DatabaseConnection,
}

impl UnitOfWorkFactory {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UnitOfWorkFactoryTrait<UnitOfWork> for UnitOfWorkFactory {
    async fn begin(self) -> Result<UnitOfWork, std::io::Error> {
        let tx = self
            .db
            .begin()
            .await
            .map_err(|e| Error::new(ErrorKind::ConnectionAborted, e))?;
        Ok(UnitOfWork::new(tx))
    }
}

pub struct UnitOfWorkSerializableFactory {
    db: DatabaseConnection,
}

impl UnitOfWorkSerializableFactory {
    pub async fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UnitOfWorkFactoryTrait<UnitOfWork> for UnitOfWorkSerializableFactory {
    async fn begin(self) -> Result<UnitOfWork, Error> {
        let tx = self
            .db
            .begin_with_config(
                Some(IsolationLevel::Serializable),
                Some(AccessMode::ReadWrite),
            )
            .await
            .map_err(|e| Error::new(ErrorKind::ConnectionAborted, e))?;
        Ok(UnitOfWork::new(tx))
    }
}
