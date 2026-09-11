# Test fixtures

## iso_3166-1.json

- Source: the Debian [iso-codes](https://salsa.debian.org/iso-codes-team/iso-codes) project (`data/iso_3166-1.json`), obtained from the packaged copy at `/usr/share/iso-codes/json/iso_3166-1.json` (iso-codes 4.18.0).
- Copyright: the iso-codes contributors.
- License: LGPL-2.1-or-later; the license text is in [`LGPL-2.1.txt`](LGPL-2.1.txt) next to this file.
- Scope: these terms apply to the fixture file alone. The published crate does not contain it (the package `include` list ships `tests/*.rs` only), so the crate's `Apache-2.0 OR MIT` licensing is unaffected.
- Used by: `tests/iso_oracle.rs`, which verifies the country table against it.
- Refresh: copy the file from a current iso-codes installation, update the version above, and review the diff.
