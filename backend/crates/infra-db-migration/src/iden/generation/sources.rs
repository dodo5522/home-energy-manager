use crate::iden::SCHEMA_GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Sources {
    Schema,
    Table,
    Source,
    Remark,
    CreatedAt,
}

impl Iden for Sources {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "sources",
                Self::Source => "source",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
