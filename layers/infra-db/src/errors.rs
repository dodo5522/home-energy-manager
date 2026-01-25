#[derive(Debug)]
pub enum Error {
    EnvVar(std::env::VarError),
    DbConnection(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EnvVar(err) => write!(f, "{}", err),
            Error::DbConnection(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::EnvVar(err) => Some(err),
            Error::DbConnection(err) => Some(err),
        }
    }
}
