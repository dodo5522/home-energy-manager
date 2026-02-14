use layer_use_case::label::LabelInOut;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LabelItem {
    /// ラベル
    pub label: String,
    /// 備考
    pub remark: String,
}

impl From<LabelInOut> for LabelItem {
    fn from(label_in_out: LabelInOut) -> Self {
        Self {
            label: label_in_out.label,
            remark: label_in_out.remark,
        }
    }
}

impl From<LabelItem> for LabelInOut {
    fn from(label_item: LabelItem) -> Self {
        Self {
            label: label_item.label,
            remark: label_item.remark,
        }
    }
}
