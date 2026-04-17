use layer_infra_db::DatabaseConnector;
use sea_orm_migration::prelude::cli;
use std::env;
use std::env::var;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = DatabaseConnector::new(
        var("DB_MIGRATOR_NAME")?,
        var("DB_MIGRATOR_PASSWORD")?,
        var("DB_HOST")?,
        var("DB_PORT")?,
        var("DB_NAME")?,
    )
    .get_url();

    env::set_var("DATABASE_URL", db_url);
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
