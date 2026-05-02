use sea_orm::DbErr;

#[derive(Debug)]
pub enum Error {
    EnvIsNotPresent(String),
    EnvIsNotUnicode(String),
    DbFailed(DbErr),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EnvIsNotPresent(env_name) => write!(f, "{} is not present", env_name),
            Error::EnvIsNotUnicode(env_name) => write!(f, "{} is not unicode", env_name),
            Error::DbFailed(err) => write!(f, "DB error with {}", err),
        }
    }
}

impl std::error::Error for Error {}
