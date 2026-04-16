use crate::value_object::Unit;
use chrono::{DateTime, Utc};

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
