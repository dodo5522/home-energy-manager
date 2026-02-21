use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UnitPostRequest {
    /// 物理量の単位
    pub unit: String,
    /// 備考
    pub remark: String,
}
