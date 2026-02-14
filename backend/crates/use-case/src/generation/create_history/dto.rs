use chrono::{DateTime, Utc};
use layer_domain::entity::HistoryEntity;
use layer_domain::value_object::Unit;

#[derive(Debug, Clone)]
pub struct CreateHistoryInput {
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

impl From<HistoryEntity> for CreateHistoryInput {
    fn from(record: HistoryEntity) -> Self {
        Self {
            value: record.value,
            unit: record.unit,
            sub_system: record.sub_system,
            label: record.label,
            monitored_at: record.monitored_at,
        }
    }
}

impl From<CreateHistoryInput> for HistoryEntity {
    fn from(input: CreateHistoryInput) -> Self {
        HistoryEntity {
            value: input.value,
            unit: input.unit,
            sub_system: input.sub_system,
            label: input.label,
            monitored_at: input.monitored_at,
        }
    }
}
