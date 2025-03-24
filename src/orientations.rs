use serde::de::Error;
use sqlx::Type;
use std::fmt::{Display, Formatter, Result as StdResult};

/// Represents the orientation of an image.
///
/// Can be one of:
/// - `Thumbnail`: Square aspect ratio
/// - `Landscape`: Width greater than height
/// - `Portrait`: Height greater than width
///
/// # Example
/// ```
/// use sizes::Orientation;
///
/// let orientation = Orientation::from("landscape");
/// assert_eq!(orientation, Orientation::Landscape);
///
/// let orientation_str = orientation.to_string();
/// assert_eq!(orientation_str, "LANDSCAPE");
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Type)]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Orientation {
    #[default]
    Thumbnail,
    Landscape,
    Portrait
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdResult {
        let variant_str = match self {
            Orientation::Thumbnail => "THUMBNAIL",
            Orientation::Landscape => "LANDSCAPE",
            Orientation::Portrait => "PORTRAIT"
        };

        write!(f, "{}", variant_str)
    }
}

impl serde::Serialize for Orientation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let variant_str = match self {
            Orientation::Thumbnail => "THUMBNAIL",
            Orientation::Landscape => "LANDSCAPE",
            Orientation::Portrait => "PORTRAIT"
        };

        serializer.serialize_str(variant_str)
    }
}

impl<'de> serde::de::Deserialize<'de> for Orientation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;

        match variant.to_lowercase().as_str() {
            "thumbnail" => Ok(Orientation::Thumbnail),
            "landscape" => Ok(Orientation::Landscape),
            "portrait" => Ok(Orientation::Portrait),
            _ => Err(Error::unknown_variant(
                &variant,
                &["THUMBNAIL", "LANDSCAPE"]
            )),
        }
    }
}

impl From<String> for Orientation {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "landscape" => Orientation::Landscape,
            "portrait" => Orientation::Portrait,
            _ => Orientation::Thumbnail,
        }
    }
}

impl From<&String> for Orientation {
    fn from(s: &String) -> Self {
        Orientation::from(s.to_string())
    }
}

impl From<&str> for Orientation {
    fn from(s: &str) -> Self {
        Orientation::from(s.to_string())
    }
}

impl From<Option<String>> for Orientation {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Orientation::from(s),
            None => Orientation::Thumbnail,
        }
    }
}