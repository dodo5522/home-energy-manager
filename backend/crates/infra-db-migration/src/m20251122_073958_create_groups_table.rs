use crate::iden::generation::Groups;
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
                    .table((Groups::Schema, Groups::Table))
                    .if_not_exists()
                    .col(string(Groups::Group).primary_key())
                    .col(string(Groups::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(Groups::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!(
            "{}.{}",
            Groups::Schema.to_string(),
            Groups::Table.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'グループ (e.g. Array, Battery, ...)';",
                    table,
                    Groups::Group.to_string()
                ),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Groups::Schema, Groups::Table))
                    .to_owned(),
            )
            .await
    }
}
