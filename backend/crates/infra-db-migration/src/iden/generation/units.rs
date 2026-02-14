use crate::iden::SCHEMA_GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Units {
    Schema,
    Table,
    Unit,
    Remark,
    CreatedAt,
}

impl Iden for Units {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "units",
                Self::Unit => "unit",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
