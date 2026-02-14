use sea_orm::DatabaseConnection;
use std::env::var;
use std::io::Error;

/// Get a database connection using operator credentials from environment variables.
///
/// # Returns
/// A `DatabaseConnection` instance connected to the database specified in the environment variables.
pub async fn get_connection() -> Result<DatabaseConnection, Error> {
    let user = var("DB_OPERATOR_NAME").map_err(Error::other)?;
    let password = var("DB_OPERATOR_PASSWORD").map_err(Error::other)?;
    layer_infra_db::get_connection(&user, &password)
        .await
        .map_err(Error::other)
}
