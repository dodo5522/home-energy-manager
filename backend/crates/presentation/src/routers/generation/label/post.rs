use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LabelPostRequest {
    /// ラベル
    pub label: String,
    /// 備考
    pub remark: String,
}
