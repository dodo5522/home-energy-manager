use crate::value_object::Unit;
use chrono::{DateTime, Utc};

/// 発電状況の識別子
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HistoryId(pub i64);

/// 発電状況エンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEntity {
    /// 物理量の値
    pub value: f32,
    /// 物理量の単位
    pub unit: Unit,
    /// 発電サブシステムの種類
    pub sub_system: String,
    /// 発電状況のラベル
    pub label: String,
    /// 発電状況の計測日時
    pub monitored_at: DateTime<Utc>,
}

impl From<HistoryId> for i64 {
    fn from(value: HistoryId) -> Self {
        value.0
    }
}

impl From<i64> for HistoryId {
    fn from(value: i64) -> Self {
        HistoryId(value)
    }
}
