//! The country table against the ISO 3166-1 source.
//!
//! `Country::countries()` is maintained by hand, and this test proves it:
//! every (alpha2, alpha3, numeric) triple is compared against a committed
//! snapshot of the Debian iso-codes project's `iso_3166-1.json`, the
//! transcription of the official ISO publication that most distributions
//! ship. An ISO amendment or a table edit then surfaces as a failing test
//! instead of drifting silently.
//!
//! The snapshot lives at `tests/fixtures/iso_3166-1.json` and stays
//! repository-only (the package `include` list ships `tests/*.rs` alone),
//! so its LGPL-2.1-or-later terms apply to that file and nothing else;
//! `tests/fixtures/README.md` carries the notice and the license text
//! sits next to it. Refresh it by copying
//! `/usr/share/iso-codes/json/iso_3166-1.json` from a current iso-codes
//! installation and reviewing the diff.

use celes::Country;
use std::collections::BTreeMap;

/// Table entries beyond current ISO 3166-1, each with its reason.
const TABLE_ONLY: &[&str] = &[
    // Kosovo: a user-assigned code with wide practical use; ISO 3166-1
    // carries no entry for it.
    "XK",
];

#[test]
fn table_matches_the_iso_3166_1_source() {
    let parsed: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/iso_3166-1.json")).expect("fixture parses");
    let official: BTreeMap<&str, (&str, usize)> = parsed["3166-1"]
        .as_array()
        .expect("fixture carries a 3166-1 array")
        .iter()
        .map(|entry| {
            (
                entry["alpha_2"].as_str().expect("alpha_2 is a string"),
                (
                    entry["alpha_3"].as_str().expect("alpha_3 is a string"),
                    entry["numeric"]
                        .as_str()
                        .expect("numeric is a string")
                        .parse::<usize>()
                        .expect("numeric is digits"),
                ),
            )
        })
        .collect();

    let table: BTreeMap<&str, (&str, usize)> = Country::countries()
        .iter()
        .map(|country| (country.alpha2, (country.alpha3, country.value)))
        .collect();

    let missing: Vec<&&str> = official
        .keys()
        .filter(|alpha2| !table.contains_key(**alpha2))
        .collect();
    assert!(
        missing.is_empty(),
        "ISO 3166-1 entries missing from the table: {missing:?}"
    );

    let undocumented: Vec<&&str> = table
        .keys()
        .filter(|alpha2| !official.contains_key(**alpha2) && !TABLE_ONLY.contains(*alpha2))
        .collect();
    assert!(
        undocumented.is_empty(),
        "table entries beyond ISO 3166-1 lack a documented reason: {undocumented:?}"
    );

    for (alpha2, expected) in &official {
        let found = table.get(alpha2).expect("presence asserted above");
        assert_eq!(found, expected, "code triple differs for {alpha2}");
    }
}
