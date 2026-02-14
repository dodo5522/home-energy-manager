use sea_orm_migration::prelude::cli;
use std::env;

use layer_infra_db::get_db_url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let user = env::var("DB_MIGRATOR_NAME")?;
    let password = env::var("DB_MIGRATOR_PASSWORD")?;
    let db_url = get_db_url(&user, &password)?;

    env::set_var("DATABASE_URL", db_url);
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
