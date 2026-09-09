# Changelog

All notable changes to Celes are documented in this file.

## [3.0.0] - 2026-09-09

### Added

- Added opt-in ISO 3166-2 subdivision support through the `subdivisions`
  feature. It provides 5,027 current codes and English names from Unicode CLDR
  48.2, allocation-free lookup, country associations, and serde support.
- Added `CountryParseError` and `SubdivisionParseError` typed errors.
- Added `Country::countries()` for borrowing the complete country table.
- Added `Country::has_alias()` for ASCII case-insensitive checks within a
  country's alias slice.
- Added `Country::turkiye()` and retained `Country::turkey()` as a deprecated
  compatibility constructor.

### Changed

- Replaced the version 2 alias-table hierarchy with static alias slices. On
  64-bit targets this reduces `Country` from 192 bytes to 88 bytes while
  retaining its `Copy` implementation.
- Changed all fallible country lookup methods and `FromStr` from string errors
  to `CountryParseError`.
- Changed `Country` ordering to use its numeric ISO 3166-1 value instead of its
  long name. Equality and hashing continue to use the numeric value.
- Made serde an optional, default-enabled feature. Builds using
  `default-features = false` no longer include serde.
- Updated Turkey's canonical name to Türkiye. `"Turkey"` and `"Turkiye"`
  remain accepted aliases.
- Made case-insensitive country lookup allocation-free.

### Removed

- Removed `CountryTable`, `LookupTable`, `EmptyLookupTable`, and the
  country-specific alias-table types. Use `Country::aliases` and
  `Country::has_alias()` instead.

### Compatibility

- The minimum supported Rust version is 1.97.
- The crate remains compatible with `no_std`. Core country functionality does
  not require allocation, and subdivision support is also allocation-free.

[3.0.0]: https://github.com/mikelodder7/celes/compare/2.8.0...v3.0.0
