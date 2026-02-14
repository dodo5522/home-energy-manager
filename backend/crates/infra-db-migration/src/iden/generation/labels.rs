use crate::iden::SCHEMA_GENERATION;
use sea_orm_migration::prelude::Iden;

pub enum Labels {
    Schema,
    Table,
    Label,
    Remark,
    CreatedAt,
}

impl Iden for Labels {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Schema => SCHEMA_GENERATION,
                Self::Table => "labels",
                Self::Label => "label",
                Self::Remark => "remark",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}
