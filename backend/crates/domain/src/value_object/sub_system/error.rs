use super::SubSystemError;

impl std::fmt::Display for SubSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubSystemError::NotSupported(s) => write!(f, "sub system '{s}' is not supported"),
        }
    }
}

impl std::error::Error for SubSystemError {}
