mod errors;
pub use errors::*;
mod connector;
pub use connector::*;
mod error_mapper;
mod models;
pub mod repository;
pub mod unit_of_work;

pub type Result<T> = std::result::Result<T, Error>;
