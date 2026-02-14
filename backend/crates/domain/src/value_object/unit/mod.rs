mod error;
mod value;

/// 単位の生成に失敗したときのエラー
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnitError {
    Empty,
    Blank,
    Invalid(String),
}

/// 物理量の単位を表す値オブジェクト
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Unit(String);
