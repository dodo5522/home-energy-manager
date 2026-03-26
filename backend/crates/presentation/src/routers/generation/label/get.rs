use layer_domain::entity::LabelEntity;
use serde::Serialize;
use std::io::ErrorKind;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LabelItem {
    /// ラベル
    pub label: String,
    /// 備考
    pub remark: String,
}

impl TryFrom<LabelEntity> for LabelItem {
    type Error = std::io::Error;

    fn try_from(e: LabelEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            label: e.label,
            remark: e
                .remark
                .ok_or_else(|| Self::Error::from(ErrorKind::InvalidInput))?,
        })
    }
}

impl From<LabelItem> for LabelEntity {
    fn from(i: LabelItem) -> Self {
        Self {
            label: i.label,
            remark: Some(i.remark),
        }
    }
}
