use crate::value_object::Unit;

/// 単位エンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct UnitRecord {
    /// 物理量の単位
    pub unit: Unit,
    /// 補足
    pub remark: String,
}
