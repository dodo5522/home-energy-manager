use super::{EnergySource, EnergySourceError};

const ENERGY_SOURCES: &[&str] = &[
    "Solar", // 太陽光
    "Wind",  // 風力
];

impl EnergySource {
    /// 発電元の新しいインスタンスを生成する
    ///
    /// # Arguments
    /// - `name` - 発電元を表す文字列
    ///
    /// # Errors
    /// - `EnergySourceError::NotSupported` - サポートされていない発電元が指定された場合
    ///
    pub fn new(name: impl Into<String>) -> Result<Self, EnergySourceError> {
        let name: String = name.into();
        if !ENERGY_SOURCES.contains(&name.as_str()) {
            Err(EnergySourceError::NotSupported(name))
        } else {
            Ok(Self(name))
        }
    }
}

impl std::fmt::Display for EnergySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<EnergySource> for String {
    fn from(value: EnergySource) -> Self {
        value.0
    }
}

impl From<&EnergySource> for String {
    fn from(value: &EnergySource) -> Self {
        value.0.to_owned()
    }
}

impl TryFrom<&str> for EnergySource {
    type Error = EnergySourceError;

    fn try_from(value: &str) -> Result<EnergySource, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for EnergySource {
    type Error = EnergySourceError;

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
        let e = EnergySource::new("Solar").unwrap();
        assert_eq!(format!("{}", &e), "Solar");
        assert_eq!(format!("{e}"), "Solar");
    }

    #[test]
    fn creation() {
        assert_eq!(EnergySource::new("Solar").unwrap().to_string(), "Solar");
        assert_eq!(EnergySource::new("Wind").unwrap().to_string(), "Wind");

        let e = EnergySource::new("");
        assert_eq!(
            format!("{}", e.err().unwrap()),
            "energy source '' is not supported"
        );
    }

    #[test]
    fn from_and_into() {
        let e = EnergySource::try_from("");
        assert!(e.is_err());

        let e = EnergySource::try_from("Wind");
        assert!(e.is_ok());
        assert_eq!(String::from(e.unwrap()), "Wind");

        let e: Result<String, Infallible> = EnergySource::try_from("Solar").unwrap().try_into();
        assert!(e.is_ok());
        assert_eq!(e.unwrap(), "Solar");
        let e: String = EnergySource::try_from("Solar").unwrap().into();
        assert_eq!(e, "Solar");
    }
}
