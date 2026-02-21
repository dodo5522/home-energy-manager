use layer_use_case::sub_system::SubSystemInOut;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SubSystemItem {
    /// 発電サブシステムの種類
    pub sub_system: String,
    /// 備考
    pub remark: String,
}

impl From<SubSystemInOut> for SubSystemItem {
    fn from(sub_system_in_out: SubSystemInOut) -> Self {
        Self {
            sub_system: sub_system_in_out.sub_system,
            remark: sub_system_in_out.remark,
        }
    }
}

impl From<SubSystemItem> for SubSystemInOut {
    fn from(sub_system_item: SubSystemItem) -> Self {
        Self {
            sub_system: sub_system_item.sub_system,
            remark: sub_system_item.remark,
        }
    }
}
