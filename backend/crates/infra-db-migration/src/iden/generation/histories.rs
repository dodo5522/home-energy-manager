use crate::iden::SCHEMA_GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Histories {
    Schema,
    Table,
    Id,
    Group,
    Source,
    Label,
    Unit,
    Value,
    Remark,
    MonitoredAt,
    CreatedAt,
    UpdatedAt,
}

impl Iden for Histories {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "histories",
                Self::Id => "id",
                Self::Group => "group",
                Self::Source => "source",
                Self::Label => "label",
                Self::Unit => "unit",
                Self::Value => "value",
                Self::Remark => "remark",
                Self::MonitoredAt => "monitored_at",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}
