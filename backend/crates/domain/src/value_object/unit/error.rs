use super::UnitError;
use std::fmt;

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
