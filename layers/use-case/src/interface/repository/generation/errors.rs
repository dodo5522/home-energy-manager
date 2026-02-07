#[derive(Debug)]
pub enum GenerationRepositoryError {
    NotImplemented,
    Domain(String),
    UseCase(String),
    Infra(String),
    Generic(String),
}

impl std::fmt::Display for GenerationRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GenerationRepositoryError::NotImplemented => write!(f, "Not implemented"),
            GenerationRepositoryError::Domain(msg) => write!(f, "Domain error: {}", msg),
            GenerationRepositoryError::UseCase(msg) => write!(f, "Use case error: {}", msg),
            GenerationRepositoryError::Infra(msg) => write!(f, "Infra error: {}", msg),
            GenerationRepositoryError::Generic(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl std::error::Error for GenerationRepositoryError {}
