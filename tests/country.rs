//! Country tests
use celes::{Country, LookupTable};
use std::str::FromStr;

#[test]
fn from_strings() {
    for c in &Country::get_countries() {
        let res = Country::from_str(&c.to_string());
        assert!(res.is_ok(), "from_str({})", c.to_string());
        assert_eq!(res.unwrap(), *c);

        let res = Country::from_name(&c.to_string());
        assert!(res.is_ok(), "from_name({})", c.to_string());
        assert_eq!(res.unwrap(), *c);

        let res = Country::from_value(c.value);
        assert!(res.is_ok(), "from_value({}) - {}", c.to_string(), c.value);
        assert_eq!(res.unwrap(), *c);

        let res = Country::from_code(&c.code);
        assert!(res.is_ok(), "from_code({}) - {}", c.to_string(), c.code);
        assert_eq!(res.unwrap(), *c);

        let res = Country::from_alpha2(&c.alpha2);
        assert!(res.is_ok(), "from_alpha2({}) - {}", c.to_string(), c.alpha2);
        assert_eq!(res.unwrap(), *c);

        let res = Country::from_alpha3(&c.alpha3);
        assert!(res.is_ok(), "from_alpha3({}) - {}", c.to_string(), c.alpha3);
        assert_eq!(res.unwrap(), *c);

        for alias in c.aliases.iter() {
            let res = Country::from_alias(*alias);
            assert!(res.is_ok(), "from_alias({}) - {}", c.to_string(), alias);
            assert_eq!(res.unwrap(), *c);
        }

        let json = serde_json::to_string(&c);
        assert!(json.is_ok(), "serde_json({})", c.to_string());
        let json = json.unwrap();
        let res: Country = serde_json::from_str(&json).unwrap();
        assert_eq!(res, *c);
    }

    let res = Country::from_str("hello");
    assert!(res.is_err());
    let res = Country::from_str("aaa");
    assert!(res.is_err());
}
