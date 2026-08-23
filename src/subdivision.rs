use core::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeserializeError, Visitor},
};

use crate::Country;

/// An error returned when a value cannot be resolved to an ISO 3166-2 subdivision.
#[derive(Copy, Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SubdivisionParseError {
    /// The ISO 3166-2 code is unknown.
    #[error("invalid subdivision code")]
    InvalidCode,
}

/// A current ISO 3166-2 country subdivision.
///
/// Resolve a subdivision with [`Subdivision::from_code`], [`FromStr`], or one
/// of the subdivision-list methods. Each resolved value contains a validated
/// country association.
#[derive(Copy, Clone)]
pub struct Subdivision {
    /// The full ISO 3166-2 code, such as `US-CA`.
    pub code: &'static str,
    /// The English subdivision name supplied by Unicode CLDR.
    pub name: &'static str,
    country_index: usize,
}

const ALPHA2_CODE_COUNT: usize = 26 * 26;

const fn letter_index(letter: u8) -> usize {
    match letter {
        b'A' => 0,
        b'B' => 1,
        b'C' => 2,
        b'D' => 3,
        b'E' => 4,
        b'F' => 5,
        b'G' => 6,
        b'H' => 7,
        b'I' => 8,
        b'J' => 9,
        b'K' => 10,
        b'L' => 11,
        b'M' => 12,
        b'N' => 13,
        b'O' => 14,
        b'P' => 15,
        b'Q' => 16,
        b'R' => 17,
        b'S' => 18,
        b'T' => 19,
        b'U' => 20,
        b'V' => 21,
        b'W' => 22,
        b'X' => 23,
        b'Y' => 24,
        b'Z' => 25,
        _ => 26,
    }
}

const fn alpha2_index(code: &str) -> usize {
    let &[first, second, ..] = code.as_bytes() else {
        return 0;
    };
    letter_index(first) * 26 + letter_index(second)
}

const fn country_indexes_by_alpha2() -> [usize; ALPHA2_CODE_COUNT] {
    let mut country_indexes = [0; ALPHA2_CODE_COUNT];
    let countries = Country::countries();
    let mut index = 0;

    while index < countries.len() {
        country_indexes[alpha2_index(countries[index].alpha2)] = index;
        index += 1;
    }

    country_indexes
}

const COUNTRY_INDEXES_BY_ALPHA2: [usize; ALPHA2_CODE_COUNT] = country_indexes_by_alpha2();

macro_rules! subdivision {
    ($code:literal, $name:literal) => {
        Subdivision {
            code: $code,
            name: $name,
            country_index: COUNTRY_INDEXES_BY_ALPHA2[alpha2_index($code)],
        }
    };
}

mod data {
    use super::{COUNTRY_INDEXES_BY_ALPHA2, Subdivision, alpha2_index};

    include!("subdivision_data.rs");
}

/// The Unicode CLDR release used for the bundled ISO 3166-2 subdivision data.
pub const SUBDIVISION_DATA_VERSION: &str = data::CLDR_VERSION;

impl Subdivision {
    /// Returns all current subdivisions, sorted by ISO 3166-2 code.
    ///
    /// # Examples
    ///
    /// ```
    /// use celes::Subdivision;
    ///
    /// let subdivisions = Subdivision::subdivisions();
    /// assert!(subdivisions.windows(2).all(|pair| pair[0] < pair[1]));
    /// assert!(subdivisions.iter().any(|value| value.code == "JP-13"));
    /// ```
    #[must_use]
    pub const fn subdivisions() -> &'static [Self] {
        &data::SUBDIVISIONS
    }

    /// Returns the country associated with this subdivision.
    ///
    /// The association is resolved when the bundled table is compiled, so this
    /// lookup requires no parsing and cannot fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use celes::{Country, Subdivision};
    /// use core::str::FromStr;
    ///
    /// let bavaria = Subdivision::from_str("DE-BY")?;
    /// assert_eq!(bavaria.country(), Country::germany());
    /// # Ok::<(), Box<dyn core::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn country(&self) -> Country {
        Country::countries()[self.country_index]
    }

    /// Resolves a full ISO 3166-2 code using ASCII case-insensitive matching.
    ///
    /// # Examples
    ///
    /// ```
    /// use celes::{Subdivision, SubdivisionParseError};
    ///
    /// let queensland = Subdivision::from_code("au-qld")?;
    /// assert_eq!(queensland.code, "AU-QLD");
    /// assert_eq!(queensland.name, "Queensland");
    /// assert_eq!(
    ///     Subdivision::from_code("AU-UNKNOWN"),
    ///     Err(SubdivisionParseError::InvalidCode)
    /// );
    /// # Ok::<(), SubdivisionParseError>(())
    /// ```
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

impl Debug for Subdivision {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter
            .debug_struct("Subdivision")
            .field("code", &self.code)
            .field("name", &self.name)
            .finish_non_exhaustive()
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
    ///
    /// Names can repeat across countries, so search within a known country's
    /// slice when resolving a subdivision by its English name.
    ///
    /// # Examples
    ///
    /// ```
    /// use celes::Country;
    ///
    /// let canada = Country::canada();
    /// let ontario = canada
    ///     .subdivisions()
    ///     .iter()
    ///     .find(|subdivision| subdivision.name == "Ontario");
    ///
    /// assert_eq!(ontario.map(|subdivision| subdivision.code), Some("CA-ON"));
    /// assert!(canada
    ///     .subdivisions()
    ///     .iter()
    ///     .all(|subdivision| subdivision.country() == canada));
    /// ```
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

#[cfg(feature = "serde")]
impl Serialize for Subdivision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.code)
    }
}

#[cfg(feature = "serde")]
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

#[cfg(test)]
mod tests {
    use super::{Country, alpha2_index, country_indexes_by_alpha2, letter_index};

    #[test]
    fn compile_time_country_indexes_are_complete() {
        for (expected, letter) in (b'A'..=b'Z').enumerate() {
            assert_eq!(letter_index(letter), expected);
        }
        assert_eq!(letter_index(b'?'), 26);
        assert_eq!(alpha2_index(""), 0);
        assert_eq!(alpha2_index("AD-02"), 3);

        let country_indexes = country_indexes_by_alpha2();
        for (expected, country) in Country::countries().iter().enumerate() {
            assert_eq!(country_indexes[alpha2_index(country.alpha2)], expected);
        }
    }
}
