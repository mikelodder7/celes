# Celes

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2 licensed][license-image]
![Rust Version][rustc-image]
![Maintenance Status: Passively-Maintained][maintenance-image]
[![Build Status][build-image]][build-link]

Convenience crate for handling ISO 3166-1

If there are any countries missing then please let me know or submit a PR

The main struct is `Country` which provides the following properties

- `code` - The three digit code for the country
- `value` - The code as an integer
- `alpha2` - The alpha2 letter set for the country
- `alpha3` - The alpha3 letter set for the country
- `long_name` - The official state name for the country
- `aliases` - Other names by which the country is known. For example,

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
- `from_alpha2` - create `Country` from two letter code
- `from_alpha3` - create `Country` from three letter code
- `from_alias` - create `Country` from a common alias. This only works for some countries as not all countries have aliases
- `from_name` - create `Country` from the full state name no space or underscores

`Country` implements the [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait that accepts any valid argument to the previously mentioned functions
such as:

- The country aliases like UnitedKingdom, GreatBritain, Russia, America
- The full country name
- The alpha2 code
- The alpha3 code

If you are uncertain which function to use, just use `Country::from_str` as it accepts
any of the valid string values. `Country::from_str` is case-insensitive

## From String Example

```rust
use celes::Country;
use std::str::FromStr;

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

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/celes.svg
[crate-link]: https://crates.io/crates/celes
[docs-image]: https://docs.rs/celes/badge.svg
[docs-link]: https://docs.rs/celes/1.0.3/celes/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.39+-blue.svg
[maintenance-image]: https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg
[build-image]: https://travis-ci.com/mikelodder7/celes.svg?branch=master
[build-link]: https://travis-ci.com/mikelodder7/celes
