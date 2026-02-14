use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct SubSystemPostRequest {
    /// 発電サブシステムの種類
    pub sub_system: String,
    /// 備考
    pub remark: String,
}
