use crate::iden::generation::Sources;
use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Sources::Schema, Sources::Table))
                    .if_not_exists()
                    .col(string(Sources::Source).primary_key())
                    .col(string(Sources::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(Sources::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!(
            "{}.{}",
            Sources::Schema.to_string(),
            Sources::Table.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '発電元ソース (e.g. Solar, Wind, ...)';",
                    table,
                    Sources::Source.to_string()
                ),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Sources::Schema, Sources::Table))
                    .to_owned(),
            )
            .await
    }
}
