use super::{Unit, UnitError};
use std::fmt;

impl Unit {
    /// 物理量単位の値オブジェクトを生成する
    ///
    /// # Arguments
    /// - `name` - 単位を表す文字列
    ///
    /// # Errors
    /// - `UnitError::Empty` - 空文字列が渡された場合
    /// - `UnitError::Blank` - 空白文字列が渡された場合
    /// - `UnitError::Invalid` - 文字列が英数字を含まない場合
    ///
    pub fn new(name: impl Into<String>) -> Result<Self, UnitError> {
        let name = name.into();
        if name.is_empty() {
            Err(UnitError::Empty)
        } else if name.trim().is_empty() {
            Err(UnitError::Blank)
        } else if name.trim().chars().all(|c| !c.is_alphanumeric()) {
            Err(UnitError::Invalid(name))
        } else {
            Ok(Self(name))
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<Unit> for String {
    fn from(u: Unit) -> Self {
        u.0
    }
}

impl From<&Unit> for String {
    fn from(u: &Unit) -> Self {
        u.0.to_owned()
    }
}

impl TryFrom<&str> for Unit {
    type Error = UnitError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Unit {
    type Error = UnitError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests_unit {
    use super::*;
    use std::convert::Infallible;

    #[test]
    fn display() {
        let unit = Unit::new("m/s").unwrap();
        assert_eq!(format!("{}", &unit), "m/s");
        assert_eq!(format!("{unit}"), "m/s");
    }

    #[test]
    fn creation() {
        assert_eq!(Unit::new("V").unwrap().to_string(), "V");
        assert_eq!(Unit::new("Ah").unwrap().to_string(), "Ah");
        assert_eq!(Unit::new("kWh").unwrap().to_string(), "kWh");
        assert_eq!(Unit::new("m/s").unwrap().to_string(), "m/s");

        let u = Unit::new("");
        assert_eq!(format!("{}", u.err().unwrap()), "unit must not be empty");
        let u = Unit::new(" ");
        assert_eq!(format!("{}", u.err().unwrap()), "unit must not be blank");
        let u = Unit::new("-");
        assert_eq!(format!("{}", u.err().unwrap()), "'-' is invalid");
    }

    #[test]
    fn from_and_into() {
        let u = Unit::try_from("");
        assert!(u.is_err());

        let u = Unit::try_from("kWh");
        assert!(u.is_ok());
        assert_eq!(String::from(u.unwrap()), "kWh");

        let u: Result<String, Infallible> = Unit::try_from("m/s").unwrap().try_into();
        assert!(u.is_ok());
        assert_eq!(u.unwrap(), "m/s");
        let u: String = Unit::try_from("m/s").unwrap().into();
        assert_eq!(u, "m/s");
    }
}
