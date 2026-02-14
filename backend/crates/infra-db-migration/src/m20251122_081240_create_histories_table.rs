use crate::iden::generation::{Groups, Histories, Labels, Sources, Units};
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
                    .table((Histories::Schema, Histories::Table))
                    .if_not_exists()
                    .col(big_integer(Histories::Id).primary_key().auto_increment())
                    .col(string(Histories::Group).not_null())
                    .col(string(Histories::Source).not_null())
                    .col(string(Histories::Label).not_null())
                    .col(string(Histories::Unit).not_null())
                    .col(float(Histories::Value).not_null())
                    .col(string(Histories::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(Histories::MonitoredAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Histories::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Histories::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-group")
                            .from((Histories::Schema, Histories::Table), Histories::Group)
                            .to((Groups::Schema, Groups::Table), Groups::Group)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-source")
                            .from((Histories::Schema, Histories::Table), Histories::Source)
                            .to((Sources::Schema, Sources::Table), Sources::Source)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-label")
                            .from((Histories::Schema, Histories::Table), Histories::Label)
                            .to((Labels::Schema, Labels::Table), Labels::Label)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-histories-unit")
                            .from((Histories::Schema, Histories::Table), Histories::Unit)
                            .to((Units::Schema, Units::Table), Units::Unit)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!(
            "{}.{}",
            Histories::Schema.to_string(),
            Histories::Table.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'グループ';",
                    table,
                    Histories::Group.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '発電元ソース';",
                    table,
                    Histories::Source.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'ラベル';",
                    table,
                    Histories::Label.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '単位';",
                    table,
                    Histories::Unit.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '値';",
                    table,
                    Histories::Value.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '観測日時';",
                    table,
                    Histories::MonitoredAt.to_string()
                ),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                r#"
                    CREATE TRIGGER updated_at_setter
                    BEFORE UPDATE ON generation.histories
                    FOR EACH ROW
                    EXECUTE FUNCTION public.set_updated_at();
                    "#,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Histories::Schema, Histories::Table))
                    .to_owned(),
            )
            .await
    }
}
