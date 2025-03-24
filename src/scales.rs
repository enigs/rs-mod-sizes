use std::fmt::{Display, Formatter, Result as StdResult};
use serde::de::Error;
use serde::Serialize;
use sqlx::Type;

/// Represents the size scale of an image.
///
/// Scales from smallest to largest:
/// - `XXSM`: Extra extra small
/// - `XSM`: Extra small
/// - `SM`: Small
/// - `MD`: Medium
/// - `LG`: Large
/// - `XLG`: Extra large
/// - `XXLG`: Extra extra large
///
/// # Example
/// ```
/// use sizes::Scale;
///
/// let scale = Scale::from("lg");
/// assert_eq!(scale, Scale::LG);
///
/// let scale_str = scale.to_string();
/// assert_eq!(scale_str, "LG");
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Type)]
#[sqlx(type_name = "TEXT")]
pub enum Scale {
    #[default]
    XXSM,
    XSM,
    SM,
    MD,
    LG,
    XLG,
    XXLG
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdResult {
        let variant_str = match self {
            Scale::XXSM => "XXSM",
            Scale::XSM => "XSM",
            Scale::SM => "SM",
            Scale::MD => "MD",
            Scale::LG => "LG",
            Scale::XLG => "XLG",
            Scale::XXLG => "XXLG"
        };

        write!(f, "{}", variant_str)
    }
}

impl Serialize for Scale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let variant_str = match self {
            Scale::XXSM => "XXSM",
            Scale::XSM => "XSM",
            Scale::SM => "SM",
            Scale::MD => "MD",
            Scale::LG => "LG",
            Scale::XLG => "XLG",
            Scale::XXLG => "XXLG"
        };

        serializer.serialize_str(variant_str)
    }
}

impl<'de> serde::de::Deserialize<'de> for Scale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;

        match variant.to_lowercase().as_str() {
            "xxsm" => Ok(Scale::XXSM),
            "xsm" => Ok(Scale::XSM),
            "sm" => Ok(Scale::SM),
            "md" => Ok(Scale::MD),
            "lg" => Ok(Scale::LG),
            "xlg" => Ok(Scale::XLG),
            "xxlg" => Ok(Scale::XXLG),
            _ => Err(Error::unknown_variant(
                &variant,
                &[ "XXSM", "XSM", "SM", "MD", "LG", "XLG", "XXLG"]
            )),
        }
    }
}

impl From<String> for Scale {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "xxsm" => Scale::XXSM,
            "xsm" => Scale::XSM,
            "sm" => Scale::SM,
            "md" => Scale::MD,
            "lg" => Scale::LG,
            "xlg" => Scale::XLG,
            "xxlg" => Scale::XXLG,
            _ => Scale::XXSM,
        }
    }
}

impl From<&String> for Scale {
    fn from(s: &String) -> Self {
        Scale::from(s.to_string())
    }
}

impl From<&str> for Scale {
    fn from(s: &str) -> Self {
        Scale::from(s.to_string())
    }
}

impl From<Option<String>> for Scale {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Scale::from(s),
            None => Scale::XXSM,
        }
    }
}