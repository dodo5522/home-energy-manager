mod error;
mod value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubSystemError {
    NotSupported(String),
}

/// 発電システム内のBattery, Arrayなどのサブシステムを表す値オブジェクト
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SubSystem(String);
