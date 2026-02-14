use super::{SubSystem, SubSystemError};

const SUB_SYSTEMS: &[&str] = &[
    "Battery",     // 蓄電池
    "Array",       // 太陽光パネル
    "WindTurbine", // 風力タービン
    "Controller",  // 充放電コントローラ
];

impl SubSystem {
    /// サブシステムの新しいインスタンスを生成する
    ///
    /// # Arguments
    /// - `name` - サブシステム名を表す文字列
    ///
    /// # Errors
    /// - `SubSystemError::NotSupported` - サポートされていサブシステム名が渡された場合
    ///
    pub fn new(name: impl Into<String>) -> Result<Self, SubSystemError> {
        let name: String = name.into();
        if !SUB_SYSTEMS.contains(&name.as_str()) {
            Err(SubSystemError::NotSupported(name))
        } else {
            Ok(Self(name))
        }
    }
}

impl std::fmt::Display for SubSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<SubSystem> for String {
    fn from(value: SubSystem) -> Self {
        value.0
    }
}

impl From<&SubSystem> for String {
    fn from(value: &SubSystem) -> Self {
        value.0.to_owned()
    }
}

impl TryFrom<&str> for SubSystem {
    type Error = SubSystemError;

    fn try_from(value: &str) -> Result<SubSystem, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for SubSystem {
    type Error = SubSystemError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    #[test]
    fn display() {
        let ss = SubSystem::new("Battery").unwrap();
        assert_eq!(format!("{}", &ss), "Battery");
        assert_eq!(format!("{ss}"), "Battery");
    }

    #[test]
    fn creation() {
        assert_eq!(SubSystem::new("Battery").unwrap().to_string(), "Battery");
        assert_eq!(SubSystem::new("Array").unwrap().to_string(), "Array");
        assert_eq!(
            SubSystem::new("WindTurbine").unwrap().to_string(),
            "WindTurbine"
        );
        assert_eq!(
            SubSystem::new("Controller").unwrap().to_string(),
            "Controller"
        );

        let ss = SubSystem::new("");
        assert_eq!(
            format!("{}", ss.err().unwrap()),
            "sub system '' is not supported"
        );
    }

    #[test]
    fn from_and_into() {
        let ss = SubSystem::try_from("");
        assert!(ss.is_err());

        let ss = SubSystem::try_from("Array");
        assert!(ss.is_ok());
        assert_eq!(String::from(ss.unwrap()), "Array");

        let ss: Result<String, Infallible> = SubSystem::try_from("Array").unwrap().try_into();
        assert!(ss.is_ok());
        assert_eq!(ss.unwrap(), "Array");
        let ss: String = SubSystem::try_from("Array").unwrap().into();
        assert_eq!(ss, "Array");
    }
}
