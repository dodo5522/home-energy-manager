use crate::errors::Error;
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
pub struct DatabaseConnector {
    user: String,
    password: String,
    db_host: String,
    db_port: String,
    db_name: String,
}

impl DatabaseConnector {
    pub fn new(
        user: String,
        password: String,
        db_host: String,
        db_port: String,
        db_name: String,
    ) -> DatabaseConnector {
        Self {
            user,
            password,
            db_host,
            db_port,
            db_name,
        }
    }

    /// Get the database URL from environment variables.
    ///
    /// # Returns
    /// A `String` representing the database URL in the format:
    /// `postgresql://{DB_USER}:{DB_PASSWORD}@{DB_HOST}:{DB_PORT}/{
    pub fn get_url(&self) -> String {
        let (user, password, db_host, db_port, db_name) = (
            &self.user,
            &self.password,
            &self.db_host,
            &self.db_port,
            &self.db_name,
        );
        format!("postgresql://{user}:{password}@{db_host}:{db_port}/{db_name}")
    }

    /// Get a database connection.
    ///
    /// # Returns
    /// A `DatabaseConnection` instance connected to the database specified in the environment variables.
    pub async fn get_connection(&self) -> Result<DatabaseConnection, Error> {
        Ok(Database::connect(self.get_url())
            .await
            .map_err(|e| Error::DbFailed(e))?)
    }
}
