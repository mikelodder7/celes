//! Subdivision tests.
#![cfg(feature = "subdivisions")]

use celes::{Country, SUBDIVISION_DATA_VERSION, Subdivision, SubdivisionParseError};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    fmt::{self, Write},
    hash::{Hash, Hasher},
    mem::size_of,
    str::FromStr,
};

#[test]
fn complete_data_supports_global_and_country_lookups() {
    let subdivisions = Subdivision::subdivisions();

    assert_eq!(SUBDIVISION_DATA_VERSION, "48.2");
    assert_eq!(subdivisions.len(), 5_027);
    assert_eq!(subdivisions.first().map(|value| value.code), Some("AD-02"));
    assert_eq!(subdivisions.last().map(|value| value.code), Some("ZW-MW"));

    let mut country_total = 0;
    for country in Country::countries() {
        let country_subdivisions = country.subdivisions();
        country_total += country_subdivisions.len();

        assert!(country_subdivisions.iter().all(|subdivision| {
            subdivision.country() == *country
                && subdivision
                    .code
                    .strip_prefix(country.alpha2)
                    .is_some_and(|suffix| suffix.starts_with('-'))
        }));
    }
    assert_eq!(country_total, subdivisions.len());

    let united_states = Country::the_united_states_of_america().subdivisions();
    assert!(united_states.iter().any(|value| value.code == "US-CA"));
    assert!(Country::antarctica().subdivisions().is_empty());
}

#[test]
fn subdivisions_resolve_their_country() -> Result<(), Box<dyn Error>> {
    let california = Subdivision::from_str("US-CA")?;

    assert_eq!(
        california.country(),
        Country::the_united_states_of_america()
    );

    Ok(())
}

#[test]
fn codes_parse_case_insensitively() -> Result<(), Box<dyn Error>> {
    let california = Subdivision::from_code("US-CA")?;

    assert_eq!(Subdivision::from_code("US-CA"), Ok(california));
    assert_eq!(Subdivision::from_code("us-ca"), Ok(california));
    assert_eq!(Subdivision::from_code("uS-cA"), Ok(california));
    assert_eq!(Subdivision::from_str("US-CA"), Ok(california));
    assert_eq!(
        Subdivision::from_code("US-ZZ"),
        Err(SubdivisionParseError::InvalidCode)
    );
    assert_eq!(
        Subdivision::from_code("TOO-LONG"),
        Err(SubdivisionParseError::InvalidCode)
    );
    assert_eq!(
        Subdivision::from_code("é"),
        Err(SubdivisionParseError::InvalidCode)
    );

    for subdivision in Subdivision::subdivisions() {
        assert_eq!(
            Subdivision::from_code(subdivision.code),
            Ok(*subdivision),
            "{}",
            subdivision.code
        );
    }

    Ok(())
}

#[test]
fn traits_use_the_canonical_code() {
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let andorra = Subdivision::from_code("AD-02");
    let california = Subdivision::from_code("US-CA");
    let lowercase_andorra = Subdivision::from_code("ad-02");

    assert!(andorra.is_ok());
    assert!(california.is_ok());
    assert!(lowercase_andorra.is_ok());

    if let (Ok(andorra), Ok(california), Ok(lowercase_andorra)) =
        (andorra, california, lowercase_andorra)
    {
        assert!(andorra < california);
        assert_eq!(andorra, lowercase_andorra);
        assert_eq!(andorra.to_string(), "AD-02");
        assert_eq!(
            format!("{andorra:?}"),
            "Subdivision { code: \"AD-02\", name: \"Canillo\", .. }"
        );
        assert_eq!(size_of::<Subdivision>(), 5 * size_of::<usize>());

        let mut hasher = DefaultHasher::new();
        andorra.hash(&mut hasher);
        assert_ne!(hasher.finish(), 0);

        assert!(write!(FailingWriter, "{andorra}").is_err());
    }
}
