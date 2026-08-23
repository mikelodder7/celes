# Celes

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
[![Downloads][downloads-image]][crate-link]
[![Coverage][coverage-image]][coverage-link]
![Maintenance Status: Passively-Maintained][maintenance-image]
![build](https://github.com/mikelodder7/celes/actions/workflows/celes.yml/badge.svg)
![MSRV][msrv-image]

Convenience crate for handling ISO 3166-1 and, optionally, ISO 3166-2 country
subdivisions. It is also compatible with `no_std` environments.

If any countries are missing, please open an issue or submit a pull request.

The minimum supported Rust version (MSRV) is 1.97.

## Features

| Feature | Default | Description |
| --- | --- | --- |
| `subdivisions` | No | Adds current ISO 3166-2 codes and English subdivision names from Unicode CLDR. |

Subdivision data is opt-in because the complete table is substantially larger
than the ISO 3166-1 country table. Default builds do not compile or include it.

## Data provenance

The country table is verified against its source: `tests/iso_oracle.rs`
compares every (alpha2, alpha3, numeric) triple with a committed snapshot of
the Debian [iso-codes](https://salsa.debian.org/iso-codes-team/iso-codes)
project's ISO 3166-1 data, so an ISO amendment or a table edit fails the test
suite instead of drifting silently. The snapshot is repository-only and not
part of the published crate; `XK` (Kosovo, a user-assigned code) is the one
documented entry beyond the standard.

The main struct is `Country`, which provides the following fields:

- `code` - The three-digit numeric code for the country.
- `value` - The numeric code as an integer.
- `alpha2` - The alpha-2 country code.
- `alpha3` - The alpha-3 country code.
- `long_name` - The official state name of the country.
- `aliases` - A static slice of other names by which the country is known.

For example, the Russian Federation is also called Russia. The United Kingdom
of Great Britain and Northern Ireland has aliases including England, Great
Britain, Northern Ireland, Scotland, and United Kingdom.

Each country can be created by calling a function whose name is the country's
name in `snake_case`.

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

Each country can also be created from a string or its numeric code. `Country`
provides several lookup methods:

- `from_code` - Creates a `Country` from a three-digit numeric code.
- `from_value` - Creates a `Country` from a numeric code represented as an
  integer.
- `from_alpha2` - Creates a `Country` from an alpha-2 code.
- `from_alpha3` - Creates a `Country` from an alpha-3 code.
- `from_alias` - Creates a `Country` from a common alias. Not all countries
  have aliases.
- `from_name` - Creates a `Country` from its full state name without spaces or
  underscores.

`Country` implements the
[`core::str::FromStr`](https://doc.rust-lang.org/core/str/trait.FromStr.html)
trait. It accepts any string identifier supported by the lookup methods above,
including:

- Country aliases such as `UnitedKingdom`, `GreatBritain`, `Russia`, and
  `America`.
- The full country name.
- The numeric code, such as `"840"`.
- The alpha-2 code.
- The alpha-3 code.

If you are uncertain which function to use, use `Country::from_str`; it accepts
all valid string identifiers and is case-insensitive.

All `from_*` lookup methods return `Result<Country, CountryParseError>`. To
check whether a specific country has an alias without performing a global
lookup, use `country.has_alias("alias")`.

## ISO 3166-2 Subdivisions

Enable subdivision support in `Cargo.toml`:

```toml
[dependencies]
celes = { version = "3", features = ["subdivisions"] }
```

The feature provides `Subdivision`, `SubdivisionParseError`,
`Subdivision::subdivisions()`, `Subdivision::country()`, and
`Country::subdivisions()`.

### Parse a Subdivision

```rust
use celes::Subdivision;
use core::str::FromStr;

fn main() {
    let california = Subdivision::from_str("US-CA").unwrap();
    assert_eq!(california.code, "US-CA");
    assert_eq!(california.name, "California");

    // Codes are parsed using ASCII case-insensitive matching.
    assert_eq!(Subdivision::from_code("us-ca").unwrap(), california);
    assert!(Subdivision::from_code("US-ZZ").is_err());
}
```

### Find a Subdivision's Country

```rust
use celes::{Country, Subdivision};
use core::str::FromStr;

fn main() {
    let california = Subdivision::from_str("US-CA").unwrap();
    assert_eq!(
        california.country(),
        Country::the_united_states_of_america()
    );
}
```

### List or Search a Country's Subdivisions

```rust
use celes::Country;

fn main() {
    let united_states = Country::the_united_states_of_america();
    let subdivisions = united_states.subdivisions();

    let new_york = subdivisions
        .iter()
        .find(|subdivision| subdivision.name == "New York")
        .unwrap();

    assert_eq!(new_york.code, "US-NY");
    assert!(subdivisions.iter().all(|subdivision| {
        subdivision.country() == united_states
    }));
}
```

### Iterate Over Every Subdivision

```rust
use celes::Subdivision;

fn main() {
    let subdivisions = Subdivision::subdivisions();

    let mut japanese_subdivisions = subdivisions
        .iter()
        .filter(|subdivision| subdivision.code.starts_with("JP-"));

    assert!(japanese_subdivisions
        .all(|subdivision| subdivision.country() == celes::Country::japan()));
}
```

`Subdivision` serializes as its canonical uppercase code and deserializes from
that code. Subdivision names are not used for parsing because many names are
not globally unique. When the country is known, search the slice returned by
`Country::subdivisions()` as shown above.

Every resolved subdivision contains a validated country association.
`Subdivision::country()` returns that `Country` directly using a precomputed
index; it performs no string parsing and cannot fail.

The bundled table contains 5,027 current subdivision codes from Unicode CLDR
48.2. Deprecated CLDR codes are excluded, entries are sorted by code, and
lookups use allocation-free binary search. The data remains compatible with
`no_std`.

### Updating the Subdivision Data

Download `common/validity/subdivision.xml` and `common/subdivisions/en.xml`
from the desired stable CLDR release, then regenerate the committed Rust table:

```sh
rustc tools/generate_subdivisions.rs -o /tmp/celes-generate-subdivisions
/tmp/celes-generate-subdivisions \
  /path/to/common/validity/subdivision.xml \
  /path/to/common/subdivisions/en.xml \
  src/subdivision_data.rs
cargo fmt --all
```

Update the generator's `CLDR_VERSION` and this README when changing CLDR
releases. The generated data is distributed under the Unicode License v3; see
`LICENSE-UNICODE`.

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
    // All three of these are equivalent.
    let usa_1 = Country::from_str("USA").unwrap();
    let usa_2 = Country::from_str("US").unwrap();
    let usa_3 = Country::from_str("America").unwrap();

    // All three of these are equivalent.
    let gb_1 = Country::from_str("England").unwrap();
    let gb_2 = Country::from_str("gb").unwrap();
    let gb_3 = Country::from_str("Scotland").unwrap();
}
```

[Documentation][docs-link]

## License

This project is available under either of the following licenses:

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](https://opensource.org/licenses/MIT)

The optional subdivision dataset is derived from Unicode CLDR and distributed
under the [Unicode License v3](LICENSE-UNICODE).

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/celes.svg
[crate-link]: https://crates.io/crates/celes
[docs-image]: https://docs.rs/celes/badge.svg
[docs-link]: https://docs.rs/celes/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[msrv-image]: https://img.shields.io/badge/rustc-1.97+-blue.svg
[maintenance-image]: https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg
[downloads-image]: https://img.shields.io/crates/d/celes.svg
[coverage-image]: https://codecov.io/gh/mikelodder7/celes/graph/badge.svg
[coverage-link]: https://codecov.io/gh/mikelodder7/celes
