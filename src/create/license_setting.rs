//! License setting primitive type

use serde::{Deserialize, Serialize};

/// License setting for model creation
///
/// Can be either a single license string or multiple licenses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LicenseSetting {
    /// Single license string
    Single(String),
    /// Multiple license strings
    Multiple(Vec<String>),
}

impl LicenseSetting {
    /// Create a single license setting
    pub fn single(license: impl Into<String>) -> Self {
        Self::Single(license.into())
    }

    /// Create a multiple licenses setting
    pub fn multiple<I, S>(licenses: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Multiple(licenses.into_iter().map(|s| s.into()).collect())
    }
}

impl From<&str> for LicenseSetting {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}

impl From<String> for LicenseSetting {
    fn from(s: String) -> Self {
        Self::Single(s)
    }
}

impl<S: Into<String>> From<Vec<S>> for LicenseSetting {
    fn from(v: Vec<S>) -> Self {
        Self::Multiple(v.into_iter().map(|s| s.into()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_setting_single() {
        let license = LicenseSetting::single("MIT");
        let json = serde_json::to_string(&license).unwrap();
        assert_eq!(json, "\"MIT\"");
    }

    #[test]
    fn test_license_setting_multiple() {
        let license = LicenseSetting::multiple(["MIT", "Apache-2.0"]);
        let json = serde_json::to_string(&license).unwrap();
        assert_eq!(json, "[\"MIT\",\"Apache-2.0\"]");
    }

    #[test]
    fn test_license_setting_from_str() {
        let license: LicenseSetting = "MIT".into();
        assert_eq!(license, LicenseSetting::Single("MIT".to_string()));
    }

    #[test]
    fn test_license_setting_deserialization_single() {
        let license: LicenseSetting = serde_json::from_str("\"MIT\"").unwrap();
        assert_eq!(license, LicenseSetting::Single("MIT".to_string()));
    }

    #[test]
    fn test_license_setting_deserialization_multiple() {
        let license: LicenseSetting = serde_json::from_str("[\"MIT\",\"Apache-2.0\"]").unwrap();
        assert_eq!(
            license,
            LicenseSetting::Multiple(vec!["MIT".to_string(), "Apache-2.0".to_string()])
        );
    }
}
