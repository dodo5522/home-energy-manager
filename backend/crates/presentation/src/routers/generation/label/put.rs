#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct UpdateLabelQuery {
    pub remark: String,
}
