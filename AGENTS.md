# AGENTS.md

A Rust library for parsing WMF (Windows Metafile) binaries and converting them
to SVG. Conforms to the
[MS-WMF specification](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2).

## Project Structure

Cargo workspace with 3 crates:

- `core/` - wmf-core: parsing (`parser` module) and conversion (`converter`
  module). Output formats are pluggable via the `Player` trait; `SVGPlayer`
  is the built-in implementation.
- `cli/` - wmf-cli: CLI tool (`cli/src/main.rs` only)
- `wasm/` - wmf-wasm: WASM bindings (`wasm/src/lib.rs` only)

See `README.md` for feature flags, CLI usage, and the WASM API. See the
`Makefile` for all build targets, including the WASM builds and the optional
Docker dev shell (`make docker-dev`).

## Constraints

- `wmf-core` and `wmf-wasm` are `#![no_std]`. Use the `alloc` crate (Vec,
  String, BTreeMap); never introduce `std` dependencies. I/O is abstracted
  via `embedded_io::Read`, re-exported at the crate root.
- All WMF data is read in little-endian byte order.
- Rust 1.88.0 is pinned via `rust-toolchain.toml`; nightly is required for
  rustfmt and cargo-udeps.

## Conventions

- All in-code text (comments, log messages, error messages) must be English.
- Errors are defined with `snafu`. Logging uses `tracing` and can be disabled
  via the `tracing` feature flag.
- WMF record type names follow the specification in `UPPER_SNAKE_CASE`
  (`non_camel_case_types` and `non_snake_case` are allowed for this reason).
- clippy runs `all` + `pedantic` at `warn` level. Format with
  `cargo +nightly fmt` (`make fmt`); line width is 80 including comments.
- When adding new WMF terms, add them to the `words` list in
  `.vscode/cspell.json` so cSpell accepts them.

## Build & Test

- Day-to-day checks: `make test`, `make lint`, `make fmt`
- Full suite before a PR: `make ci-suite`
- CI on PRs and pushes to master runs `make fmt`, `make lint`,
  `make spell-check`, and `make test`
- `make install-tools` bulk-installs the host tooling

## Testing

- Inline tests (`#[cfg(test)]`) live alongside the implementation under
  `core/src/`; shared helpers are in the `test_helpers` module in
  `core/src/parser/records/mod.rs`.
- Integration tests live in `core/tests/` with `core/tests/mod.rs` as the
  entry point.

## Releases

- Main branch: `master` (direct pushes are forbidden; changes land via PR)
- `make release version=<x.y.z>` creates a `release/<x.y.z>` branch from
  `master`, bumps `[workspace.package].version` and dependent version
  requirements via `cargo release version`, then refreshes `Cargo.lock`.
  Commit the result through a normal PR.
- When the bump lands on `master`, `.github/workflows/tag-release.yaml`
  creates the matching bare `<version>` tag and invokes
  `.github/workflows/release.yaml` in the same run (a tag pushed with
  `GITHUB_TOKEN` cannot trigger workflows). The release workflow fails if
  the version does not equal the workspace version, then publishes the
  WASM bundles as GitHub Releases assets.
- All crates share the single version in `[workspace.package]` and are
  released in lockstep: the release tag must equal that version, and
  `wmf-core` is published to crates.io with the same version. `wmf-cli` and
  `wmf-wasm` keep `publish = false` until their first crates.io release is
  prepared.
