#[derive(Debug)]
pub enum GenerationError {
    NotImplemented(String),
    Unknown(String),
    DbError(String),
    InvalidUnit(String),
    NotFound(String),
}

impl std::fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotImplemented(target) => write!(f, "{target} is not implemented yet"),
            Self::Unknown(msg) => write!(f, "Unknown error with: {msg}"),
            Self::DbError(msg) => write!(f, "Database error with: {msg}"),
            Self::InvalidUnit(unit) => write!(f, "Invalid unit: {unit}"),
            Self::NotFound(id) => write!(f, "Not found: {id}"),
        }
    }
}

impl std::error::Error for GenerationError {}
