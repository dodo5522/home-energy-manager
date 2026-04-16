use layer_domain::entity::SubSystemEntity;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SubSystemItem {
    /// 発電サブシステムの種類
    pub sub_system: String,
    /// 備考
    pub remark: String,
}

impl From<SubSystemEntity> for SubSystemItem {
    fn from(e: SubSystemEntity) -> Self {
        Self {
            sub_system: e.sub_system,
            remark: e.remark,
        }
    }
}

impl From<SubSystemItem> for SubSystemEntity {
    fn from(sub_system_item: SubSystemItem) -> Self {
        Self {
            sub_system: sub_system_item.sub_system,
            remark: sub_system_item.remark,
        }
    }
}
