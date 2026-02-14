use super::EnergySourceError;

impl std::fmt::Display for EnergySourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergySourceError::NotSupported(s) => write!(f, "energy source '{s}' is not supported"),
        }
    }
}

impl std::error::Error for EnergySourceError {}
