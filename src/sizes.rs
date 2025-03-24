use serde::{Deserialize, Serialize};

use crate::{Orientation, Scale};

/// Represents an image size with scale, orientation, and dimensions.
///
/// # Example
/// ```
/// use sizes::{Size, Scale};
///
/// let thumbnail = Size::new_thumbnail(100, Scale::MD);
/// let landscape = Size::new_landscape(1920, 1080, Scale::LG);
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub scale: Scale,
    pub orientation: Orientation,
    pub width: i32,
    pub height: i32
}

impl Size {
    /// Creates a new square thumbnail with the specified size and scale.
    ///
    /// # Example
    /// ```
    /// use sizes::{Orientation, Size, Scale};
    ///
    /// let icon = Size::new_thumbnail(64, Scale::SM);
    /// assert_eq!(icon.width, 64);
    /// assert_eq!(icon.height, 64);
    /// assert_eq!(icon.orientation, Orientation::Thumbnail);
    /// ```
    pub fn new_thumbnail(sz: i32, scale: Scale) -> Self {
        Self {
            scale,
            orientation: Orientation::Thumbnail,
            width: sz,
            height: sz
        }
    }

    /// Creates a new landscape-oriented image with the specified width, height, and scale.
    ///
    /// # Example
    /// ```
    /// use sizes::{Orientation, Size, Scale};
    ///
    /// let banner = Size::new_landscape(1200, 400, Scale::LG);
    /// assert_eq!(banner.orientation, Orientation::Landscape);
    /// ```
    pub fn new_landscape(w: i32, h: i32, scale: Scale) -> Self {
        Self {
            scale,
            orientation: Orientation::Landscape,
            width: w,
            height: h
        }
    }

    /// Creates a new portrait-oriented image with the specified width, height, and scale.
    ///
    /// # Example
    /// ```
    /// use sizes::{Orientation, Size, Scale};
    ///
    /// let poster = Size::new_portrait(800, 1200, Scale::MD);
    /// assert_eq!(poster.orientation, Orientation::Portrait);
    /// ```
    pub fn new_portrait(w: i32, h: i32, scale: Scale) -> Self {
        Self {
            scale,
            orientation: Orientation::Portrait,
            width: w,
            height: h
        }
    }

    /// Checks if the Size is the default (uninitialized) value.
    ///
    /// # Example
    /// ```
    /// use sizes::{Size, Scale};
    ///
    /// let empty_size = Size::default();
    /// assert!(empty_size.is_empty());
    ///
    /// let real_size = Size::new_thumbnail(100, Scale::MD);
    /// assert!(!real_size.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

}

impl sqlx::Type<sqlx::Postgres> for Size {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Size {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn serde::ser::StdError + Send + Sync + 'static>> {
        <sqlx::types::Json<&Self> as sqlx::Encode<'q, sqlx::Postgres>>::encode(sqlx::types::Json(self), buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Size {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = value.as_str()?
            .strip_prefix('\u{1}')
            .unwrap_or(value.as_str()?);

        Ok(serde_json::from_str(bytes)?)
    }
}