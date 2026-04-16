use crate::interface::GenerationError;
use layer_domain::value_object::UnitError;

pub trait ErrorMapperTrait {
    fn map_err_unit_value(e: UnitError) -> GenerationError {
        GenerationError::InvalidUnit(format!("{e}"))
    }

    fn map_db_err(e: std::io::Error) -> GenerationError {
        GenerationError::DbError(format!("{e}"))
    }
}
