use layer_domain::entity::LabelEntity;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LabelInOut {
    /// 発電状況のラベル
    pub label: String,
    /// 補足
    pub remark: String,
}

impl From<LabelEntity> for LabelInOut {
    fn from(record: LabelEntity) -> Self {
        Self {
            label: record.label,
            remark: record.remark,
        }
    }
}

impl From<LabelInOut> for LabelEntity {
    fn from(input: LabelInOut) -> Self {
        LabelEntity {
            label: input.label,
            remark: input.remark,
        }
    }
}
