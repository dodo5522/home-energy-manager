use crate::models::labels::{ActiveModel, Model};
use layer_domain::entity::LabelEntity;
use sea_orm::ActiveValue;

impl From<&LabelEntity> for ActiveModel {
    fn from(e: &LabelEntity) -> Self {
        Self {
            label: ActiveValue::Set(e.label.to_owned()),
            remark: ActiveValue::Set(e.remark.to_owned().unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl From<Model> for LabelEntity {
    fn from(m: Model) -> Self {
        Self {
            label: m.label,
            remark: Some(m.remark),
        }
    }
}
