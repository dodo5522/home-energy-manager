#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct UpdateUnitQuery {
    pub remark: String,
}
