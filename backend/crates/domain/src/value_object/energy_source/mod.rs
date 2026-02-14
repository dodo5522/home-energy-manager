mod error;
mod value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnergySourceError {
    NotSupported(String),
}

/// 発電システム内の発電元を表す値オブジェクト
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EnergySource(String);
