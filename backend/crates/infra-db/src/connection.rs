use super::{Error, Result};
use sea_orm::{Database, DatabaseConnection};
use std::env::var;

/// Get the database URL from environment variables.
///
/// # Arguments
/// - user: A reference to a `String` representing the database user.
/// - password: A reference to a `String` representing the database password.
///
/// # Returns
/// A `String` representing the database URL in the format:
/// `postgresql://{DB_USER}:{DB_PASSWORD}@{DB_HOST}:{DB_PORT}/{
pub fn get_db_url(user: &String, password: &String) -> Result<String> {
    let db_host = var("DB_HOST").map_err(Error::EnvVar)?;
    let db_port = var("DB_PORT").map_err(Error::EnvVar)?;
    let db_name = var("DB_NAME").map_err(Error::EnvVar)?;
    Ok(format!(
        "postgresql://{user}:{password}@{db_host}:{db_port}/{db_name}"
    ))
}

/// Get a database connection.
///
/// # Returns
/// A `DatabaseConnection` instance connected to the database specified in the environment variables.
pub async fn get_connection(user: &String, password: &String) -> Result<DatabaseConnection> {
    let db_url = get_db_url(user, password)?;
    let connection = Database::connect(db_url)
        .await
        .map_err(|e| Error::DbConnection(std::io::Error::other(e)))?;
    Ok(connection)
}
