//! Serialization round trips, behind the `serde` feature.
#![cfg(feature = "serde")]

use celes::Country;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct CountryDocument {
    country: Country,
}

#[test]
fn every_country_round_trips_as_json() -> Result<(), serde_json::Error> {
    for c in Country::countries() {
        let json = serde_json::to_string(c)?;
        let res: Country = serde_json::from_str(&json)?;
        assert_eq!(res, *c);
    }

    assert!(serde_json::from_str::<Country>("42").is_err());
    assert!(serde_json::from_str::<Country>("\"ZZ\"").is_err());

    Ok(())
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

#[cfg(feature = "subdivisions")]
mod subdivisions {
    use celes::Subdivision;
    use serde::{Deserialize, Serialize};
    use std::{error::Error, str::FromStr};

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct SubdivisionDocument {
        subdivision: Subdivision,
    }

    #[test]
    fn serialization_formats_round_trip() -> Result<(), Box<dyn Error>> {
        let expected = SubdivisionDocument {
            subdivision: Subdivision::from_str("US-CA")?,
        };

        let json = serde_json::to_string(&expected)?;
        assert_eq!(json, r#"{"subdivision":"US-CA"}"#);
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

        assert!(serde_json::from_str::<Subdivision>("42").is_err());
        assert!(serde_json::from_str::<Subdivision>("\"US-ZZ\"").is_err());

        Ok(())
    }
}
