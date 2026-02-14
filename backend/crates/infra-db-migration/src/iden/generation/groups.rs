use crate::iden::SCHEMA_GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Groups {
    Schema,
    Table,
    Group,
    Remark,
    CreatedAt,
}

impl Iden for Groups {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "groups",
                Self::Group => "group",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
