use core::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    str::FromStr,
};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeserializeError, Visitor},
};

use crate::Country;

mod data {
    use super::Subdivision;

    include!("subdivision_data.rs");
}

/// The Unicode CLDR release used for the bundled ISO 3166-2 subdivision data.
pub const SUBDIVISION_DATA_VERSION: &str = data::CLDR_VERSION;

/// An error returned when a value cannot be resolved to an ISO 3166-2 subdivision.
#[derive(Copy, Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SubdivisionParseError {
    /// The ISO 3166-2 code is unknown.
    #[error("invalid subdivision code")]
    InvalidCode,
}

/// A current ISO 3166-2 country subdivision.
#[derive(Copy, Clone, Debug)]
pub struct Subdivision {
    /// The full ISO 3166-2 code, such as `US-CA`.
    pub code: &'static str,
    /// The English subdivision name supplied by Unicode CLDR.
    pub name: &'static str,
}

impl Subdivision {
    /// Returns all current subdivisions, sorted by ISO 3166-2 code.
    #[must_use]
    pub const fn subdivisions() -> &'static [Self] {
        &data::SUBDIVISIONS
    }

    /// Resolves a full ISO 3166-2 code using ASCII case-insensitive matching.
    ///
    /// # Errors
    ///
    /// Returns [`SubdivisionParseError::InvalidCode`] when `code` is not a
    /// current ISO 3166-2 code.
    pub fn from_code<A: AsRef<str>>(code: A) -> Result<Self, SubdivisionParseError> {
        let code = code.as_ref();
        let mut normalized = [0_u8; 6];
        let normalized = normalized
            .get_mut(..code.len())
            .ok_or(SubdivisionParseError::InvalidCode)?;
        normalized.copy_from_slice(code.as_bytes());
        normalized.make_ascii_uppercase();
        let normalized =
            core::str::from_utf8(normalized).map_err(|_| SubdivisionParseError::InvalidCode)?;

        data::SUBDIVISIONS
            .binary_search_by(|subdivision| subdivision.code.cmp(normalized))
            .ok()
            .and_then(|index| data::SUBDIVISIONS.get(index))
            .copied()
            .ok_or(SubdivisionParseError::InvalidCode)
    }
}

impl Ord for Subdivision {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code.cmp(other.code)
    }
}

impl PartialOrd for Subdivision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Subdivision {}

impl PartialEq for Subdivision {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Hash for Subdivision {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl Country {
    /// Returns this country's current ISO 3166-2 subdivisions.
    #[must_use]
    pub fn subdivisions(&self) -> &'static [Subdivision] {
        let start = data::SUBDIVISIONS.partition_point(|subdivision| {
            subdivision.code.get(..2).unwrap_or_default() < self.alpha2
        });
        let end = data::SUBDIVISIONS.partition_point(|subdivision| {
            subdivision.code.get(..2).unwrap_or_default() <= self.alpha2
        });

        data::SUBDIVISIONS.get(start..end).unwrap_or_default()
    }
}

impl Display for Subdivision {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str(self.code)
    }
}

impl FromStr for Subdivision {
    type Err = SubdivisionParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Self::from_code(code)
    }
}

impl Serialize for Subdivision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.code)
    }
}

impl<'de> Deserialize<'de> for Subdivision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SubdivisionVisitor;

        impl Visitor<'_> for SubdivisionVisitor {
            type Value = Subdivision;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
                formatter.write_str("an ISO 3166-2 subdivision code")
            }

            fn visit_str<E>(self, code: &str) -> Result<Self::Value, E>
            where
                E: DeserializeError,
            {
                Subdivision::from_code(code).map_err(|_| {
                    DeserializeError::invalid_value(serde::de::Unexpected::Str(code), &self)
                })
            }
        }

        deserializer.deserialize_str(SubdivisionVisitor)
    }
}
