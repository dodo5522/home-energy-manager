mod connection;
pub use connection::*;
mod errors;
mod models;
pub mod repository;
pub use errors::*;

pub type Result<T> = std::result::Result<T, Error>;
