use layer_use_case::unit::UnitInOut;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UnitItem {
    /// 物理量の単位
    pub unit: String,
    /// 備考
    pub remark: String,
}

impl From<UnitInOut> for UnitItem {
    fn from(unit_in_out: UnitInOut) -> Self {
        Self {
            unit: unit_in_out.unit.into(),
            remark: unit_in_out.remark,
        }
    }
}

impl From<UnitItem> for UnitInOut {
    fn from(unit_item: UnitItem) -> Self {
        Self {
            unit: unit_item.unit.try_into().unwrap(),
            remark: unit_item.remark,
        }
    }
}
