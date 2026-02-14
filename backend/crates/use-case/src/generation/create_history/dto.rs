use chrono::{DateTime, Utc};
use layer_domain::entity::HistoryRecord;
use layer_domain::value_object::{EnergySource, SubSystem, Unit};

#[derive(Debug, Clone)]
pub struct CreateHistoryInput {
    /// 物理量の値
    pub value: f32,
    /// 物理量の単位
    pub unit: Unit,
    /// 発電サブシステムの種類
    pub sub_system: SubSystem,
    /// エネルギー源の種類
    pub energy_source: EnergySource,
    /// 発電状況のラベル
    pub label: String,
    /// 発電状況の計測日時
    pub monitored_at: DateTime<Utc>,
}

impl From<HistoryRecord> for CreateHistoryInput {
    fn from(record: HistoryRecord) -> Self {
        Self {
            value: record.value,
            unit: record.unit,
            sub_system: record.sub_system,
            energy_source: record.energy_source,
            label: record.label,
            monitored_at: record.monitored_at,
        }
    }
}

impl From<CreateHistoryInput> for HistoryRecord {
    fn from(input: CreateHistoryInput) -> Self {
        HistoryRecord {
            id: None,
            value: input.value,
            unit: input.unit,
            sub_system: input.sub_system,
            energy_source: input.energy_source,
            label: input.label,
            monitored_at: input.monitored_at,
        }
    }
}
