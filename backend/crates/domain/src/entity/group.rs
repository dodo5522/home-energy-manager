use crate::value_object::SubSystem;

/// グループエンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct GroupRecord {
    /// サブシステム
    pub sub_system: SubSystem,
    /// 補足
    pub remark: String,
}
