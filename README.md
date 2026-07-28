# Celes

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
[![Downloads][downloads-image]][crate-link]
[![Coverage][coverage-image]][coverage-link]
![Maintenance Status: Passively-Maintained][maintenance-image]
![build](https://github.com/mikelodder7/celes/actions/workflows/celes.yml/badge.svg)
![MSRV][msrv-image]

Convenience crate for handling ISO 3166-1. Also compatible with `no-std` environments.

If there are any countries missing then please let me know or submit a PR

The main struct is `Country` which provides the following properties

- `code` - The three digit code for the country
- `value` - The code as an integer
- `alpha2` - The alpha2 letter set for the country
- `alpha3` - The alpha3 letter set for the country
- `long_name` - The official state name for the country
- `aliases` - A static slice of other names by which the country is known. For example,

The Russian Federation is also called Russia or The United Kingdom of Great Britain
and Northern Ireland is also called England, Great Britain,
Northern Ireland, Scotland, and United Kingdom.

Each country can be instantiated by using a function with the country name in snake case

## Usage

```rust
use celes::Country;

fn main() {
     let gb = Country::the_united_kingdom_of_great_britain_and_northern_ireland();
     println!("{}", gb);

     let usa = Country::the_united_states_of_america();
     println!("{}", usa);
}
```

Additionally, each country can be created from a string or its numeric code.
`Country` provides multiple from methods to instantiate it from a string:

- `from_code` - create `Country` from three digit code
- `from_value` - create `Country` from the numeric code as an integer
- `from_alpha2` - create `Country` from two letter code
- `from_alpha3` - create `Country` from three letter code
- `from_alias` - create `Country` from a common alias. This only works for some countries as not all countries have aliases
- `from_name` - create `Country` from the full state name no space or underscores

`Country` implements the [core::str::FromStr](https://doc.rust-lang.org/core/str/trait.FromStr.html) trait that accepts any valid argument to the previously mentioned functions
such as:

- The country aliases like UnitedKingdom, GreatBritain, Russia, America
- The full country name
- The numeric code (e.g. "840")
- The alpha2 code
- The alpha3 code

If you are uncertain which function to use, just use `Country::from_str` as it accepts
any of the valid string values. `Country::from_str` is case-insensitive

All lookup methods return `Result<Country, CountryParseError>`. To check whether a
specific country has an alias without performing a global lookup, use
`country.has_alias("alias")`.

## Version 3 Alias Representation

Version 3 replaces the alias-table type hierarchy from version 2 with static
slices. A country now stores its aliases directly:

```rust
pub aliases: &'static [&'static str]
```

Version 2 used a large `CountryTable` enum plus a separate wrapper type for each
country with aliases, such as `AmericaTable` and `EnglandTable`. Those types also
required repeated implementations for iteration, comparison, formatting,
hashing, and serialization. They have been removed in version 3.

| Version 2 | Version 3 |
| --- | --- |
| `CountryTable` enum | `&'static [&'static str]` |
| Country-specific table structs | Static alias slices |
| `LookupTable::contains` | `Country::has_alias` |
| String errors | `CountryParseError` |

This change does not remove the perfect-hash lookup maps. The maps used by
`from_value`, `from_code`, `from_alpha2`, `from_alpha3`, `from_alias`,
`from_name`, and `FromStr` remain compile-time static maps. Global country
lookups therefore retain their constant-time behavior.

The new representation:

- Requires no heap allocation.
- Keeps `Country` as a `Copy` type.
- Reduces `Country` from 192 bytes to 88 bytes on 64-bit targets.
- Removes the dispatch and storage overhead of the old enum.
- Lets aliases be accessed using normal slice operations.

### Migrating from Version 2

Iterating over aliases remains straightforward:

```rust
let country = celes::Country::the_united_states_of_america();

for alias in country.aliases {
    println!("{alias}");
}
```

Replace table-specific or `LookupTable` alias checks with `has_alias`:

```rust
let country = celes::Country::the_united_states_of_america();

assert!(country.has_alias("america"));
assert!(country.has_alias("AMERICA"));
```

`has_alias` performs ASCII case-insensitive matching within one country. Use
`Country::from_alias` when the country itself is not already known:

```rust
use celes::{Country, CountryParseError};

fn main() -> Result<(), CountryParseError> {
    let country = Country::from_alias("America")?;
    assert_eq!(country, Country::the_united_states_of_america());
    Ok(())
}
```

Code importing `CountryTable`, `LookupTable`, `EmptyLookupTable`, or an
individual country table must remove those imports and use the static
`aliases` slice or `Country::has_alias`.

## From String Example

```rust
use celes::Country;
use core::str::FromStr;

fn main() {
     // All three of these are equivalent
     let usa_1 = Country::from_str("USA").unwrap();
     let usa_2 = Country::from_str("US").unwrap();
     let usa_3 = Country::from_str("America").unwrap();

     // All three of these are equivalent
     let gb_1 = Country::from_str("England").unwrap();
     let gb_2 = Country::from_str("gb").unwrap();
     let gb_3 = Country::from_str("Scotland").unwrap();
}
```


[Documentation][docs-link]

## License

Licensed under

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/celes.svg
[crate-link]: https://crates.io/crates/celes
[docs-image]: https://docs.rs/celes/badge.svg
[docs-link]: https://docs.rs/celes/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[msrv-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
[maintenance-image]: https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg
[downloads-image]: https://img.shields.io/crates/d/celes.svg
[coverage-image]: https://codecov.io/gh/mikelodder7/celes/graph/badge.svg
[coverage-link]: https://codecov.io/gh/mikelodder7/celes
