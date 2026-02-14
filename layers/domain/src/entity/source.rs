use crate::value_object::EnergySource;

/// 発電ソースエンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct SourceRecord {
    /// 発電ソース
    pub source: EnergySource,
    /// 補足
    pub remark: String,
}
