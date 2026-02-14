use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Response {
    /// 発電状況の識別子
    pub id: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct Request {
    /// 物理量の値
    pub value: f32,
    /// 物理量の単位
    pub unit: String,
    /// 発電サブシステムの種類
    pub sub_system: String,
    /// エネルギー源の種類
    pub energy_source: String,
    /// 発電状況のラベル
    pub label: String,
    /// 発電状況の計測日時
    pub monitored_at: DateTime<Utc>,
}
