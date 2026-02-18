use layer_domain::entity::SubSystemEntity;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubSystemInOut {
    /// 発電状況のラベル
    pub sub_system: String,
    /// 補足
    pub remark: String,
}

impl From<SubSystemEntity> for SubSystemInOut {
    fn from(record: SubSystemEntity) -> Self {
        Self {
            sub_system: record.sub_system,
            remark: record.remark,
        }
    }
}

impl From<SubSystemInOut> for SubSystemEntity {
    fn from(input: SubSystemInOut) -> Self {
        SubSystemEntity {
            sub_system: input.sub_system,
            remark: input.remark,
        }
    }
}
