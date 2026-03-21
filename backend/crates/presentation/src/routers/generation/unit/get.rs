use layer_domain::{entity::UnitEntity, value_object::UnitError};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UnitItem {
    /// 物理量の単位
    pub unit: String,
    /// 備考
    pub remark: String,
}

impl From<UnitEntity> for UnitItem {
    fn from(u: UnitEntity) -> Self {
        Self {
            unit: u.unit.into(),
            remark: u.remark,
        }
    }
}

impl From<&UnitEntity> for UnitItem {
    fn from(u: &UnitEntity) -> Self {
        let UnitEntity { unit, remark } = u;

        Self {
            unit: unit.into(),
            remark: remark.into(),
        }
    }
}

impl TryFrom<UnitItem> for UnitEntity {
    type Error = UnitError;

    fn try_from(u: UnitItem) -> Result<Self, Self::Error> {
        let UnitItem { unit, remark } = u;

        Ok(Self {
            unit: unit
                .clone()
                .try_into()
                .map_err(|_| UnitError::Invalid(unit))?,
            remark,
        })
    }
}

impl TryFrom<&UnitItem> for UnitEntity {
    type Error = UnitError;

    fn try_from(u: &UnitItem) -> Result<Self, Self::Error> {
        let UnitItem { unit, remark } = u;

        Ok(Self {
            unit: unit
                .try_into()
                .map_err(|_| UnitError::Invalid(unit.clone()))?,
            remark: remark.to_owned(),
        })
    }
}
