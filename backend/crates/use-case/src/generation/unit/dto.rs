use layer_domain::{entity::UnitEntity, value_object::Unit};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnitInOut {
    /// 物理量の単位
    pub unit: Unit,
    /// 補足
    pub remark: String,
}

impl From<UnitInOut> for UnitEntity {
    fn from(input: UnitInOut) -> Self {
        Self {
            unit: input.unit,
            remark: input.remark,
        }
    }
}

impl From<UnitEntity> for UnitInOut {
    fn from(entity: UnitEntity) -> Self {
        Self {
            unit: entity.unit,
            remark: entity.remark,
        }
    }
}
