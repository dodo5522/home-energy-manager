use std::fmt;

/// 単位の生成に失敗したときのエラー
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnitError {
    Empty,
    Blank,
    Invalid(String),
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitError::Empty => f.write_str("unit must not be empty"),
            UnitError::Blank => f.write_str("unit must not be blank"),
            UnitError::Invalid(unit) => write!(f, "'{unit}' is invalid"),
        }
    }
}

impl std::error::Error for UnitError {}
