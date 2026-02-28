use crate::models::labels::ActiveModel;
use layer_domain::entity::LabelEntity;
use sea_orm::ActiveValue;

impl From<&LabelEntity> for ActiveModel {
    fn from(e: &LabelEntity) -> Self {
        ActiveModel {
            label: ActiveValue::Set(e.label.to_owned()),
            remark: ActiveValue::Set(e.remark.to_owned().unwrap_or_default()),
            ..Default::default()
        }
    }
}
