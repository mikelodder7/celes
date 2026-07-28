//! Country tests
use celes::{Country, CountryParseError};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    error::Error,
    fmt::{self, Write},
    hash::{Hash, Hasher},
    mem::size_of,
    str::FromStr,
};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct CountryDocument {
    country: Country,
}

#[test]
fn from_strings() -> Result<(), serde_json::Error> {
    for c in Country::countries() {
        let name = c.to_string();

        assert_eq!(Country::from_str(&name), Ok(*c), "from_str({c})");
        assert_eq!(Country::from_name(&name), Ok(*c), "from_name({c})");
        assert_eq!(
            Country::from_value(c.value),
            Ok(*c),
            "from_value({c}) - {}",
            c.value
        );
        assert_eq!(
            Country::from_code(c.code),
            Ok(*c),
            "from_code({c}) - {}",
            c.code
        );
        assert_eq!(
            Country::from_alpha2(c.alpha2),
            Ok(*c),
            "from_alpha2({c}) - {}",
            c.alpha2
        );
        assert_eq!(
            Country::from_alpha3(c.alpha3),
            Ok(*c),
            "from_alpha3({c}) - {}",
            c.alpha3
        );

        for alias in c.aliases {
            assert_eq!(
                Country::from_alias(*alias),
                Ok(*c),
                "from_alias({c}) - {alias}"
            );
        }

        let json = serde_json::to_string(c)?;
        let res: Country = serde_json::from_str(&json)?;
        assert_eq!(res, *c);
    }

    assert_eq!(
        Country::from_str("hello"),
        Err(CountryParseError::UnknownIdentifier)
    );
    assert_eq!(Country::from_value(1), Err(CountryParseError::InvalidValue));
    assert_eq!(
        Country::from_code("01"),
        Err(CountryParseError::InvalidCode)
    );
    assert_eq!(
        Country::from_alpha2("U"),
        Err(CountryParseError::InvalidAlpha2)
    );
    assert_eq!(
        Country::from_alpha3("US"),
        Err(CountryParseError::InvalidAlpha3)
    );
    assert_eq!(
        Country::from_alias("unknown"),
        Err(CountryParseError::InvalidAlias)
    );
    assert_eq!(
        Country::from_name("UnitedStatesOfAmerica"),
        Err(CountryParseError::InvalidName)
    );

    Ok(())
}

#[test]
fn aliases_are_ascii_case_insensitive() {
    let country = Country::heard_island_and_mc_donald_islands();

    assert!(country.has_alias("mcdonaldislands"));
    assert!(country.has_alias("MCDONALDISLANDS"));
    assert!(!country.has_alias("Australia"));
}

#[test]
fn country_uses_static_alias_slice() {
    assert_eq!(size_of::<Country>(), 11 * size_of::<usize>());
}

#[test]
fn trait_and_compatibility_paths() {
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let afghanistan = Country::afghanistan();
    let aland_islands = Country::aland_islands();

    assert_eq!(
        afghanistan.partial_cmp(&aland_islands),
        Some(Ordering::Less)
    );
    assert_eq!(
        format!("{afghanistan:?}"),
        "Country { code: \"004\", value: 4, alpha2: \"AF\", alpha3: \"AFG\", \
         long_name: \"Afghanistan\", aliases: [] }"
    );

    let mut hasher = DefaultHasher::new();
    afghanistan.hash(&mut hasher);
    assert_ne!(hasher.finish(), 0);

    assert!(write!(FailingWriter, "{afghanistan}").is_err());
    assert_eq!(Country::get_countries().as_slice(), Country::countries());

    #[expect(deprecated, reason = "exercise the v2 compatibility constructor")]
    let turkey = Country::turkey();
    assert_eq!(turkey, Country::turkiye());
}

#[test]
fn invalid_and_oversized_inputs_are_rejected() {
    let oversized = "A".repeat(65);

    assert_eq!(
        Country::from_str(&oversized),
        Err(CountryParseError::UnknownIdentifier)
    );
    assert!(serde_json::from_str::<Country>("42").is_err());
    assert!(serde_json::from_str::<Country>("\"ZZ\"").is_err());
}

#[test]
fn serialization_formats_round_trip() -> Result<(), Box<dyn Error>> {
    let expected = CountryDocument {
        country: Country::the_united_states_of_america(),
    };

    let json = serde_json::to_string(&expected)?;
    let json_value = serde_json::from_str(&json)?;
    assert_eq!(expected, json_value);

    let postcard = postcard::to_allocvec(&expected)?;
    let postcard_value = postcard::from_bytes(&postcard)?;
    assert_eq!(expected, postcard_value);

    let cbor = serde_cbor_2::to_vec(&expected)?;
    let cbor_value = serde_cbor_2::from_slice(&cbor)?;
    assert_eq!(expected, cbor_value);

    let toml = toml::to_string(&expected)?;
    let toml_value = toml::from_str(&toml)?;
    assert_eq!(expected, toml_value);

    let yaml = yaml_serde::to_string(&expected)?;
    let yaml_value = yaml_serde::from_str(&yaml)?;
    assert_eq!(expected, yaml_value);

    Ok(())
}
