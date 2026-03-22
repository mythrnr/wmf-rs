# AGENTS.md

A Rust library for parsing WMF (Windows Metafile) binaries and converting them to SVG.
Conforms to the [MS-WMF specification](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2).

## Project Structure

The project is managed as a Cargo workspace with 3 crates.

```
wmf-rs/
  Cargo.toml          # Workspace root (resolver = "3")
  core/               # wmf-core: WMF parsing & SVG conversion library (no_std)
  cli/                # wmf-cli: CLI tool (example usage of wmf-core)
  wasm/               # wmf-wasm: WASM bindings (no_std, wasm-bindgen)
  docker/             # Development Docker environment
  wasm/dist/          # wasm-pack build artifacts
```

### wmf-core (Main Library)

- `#![no_std]` compatible. Uses the `alloc` crate.
- Re-exports the `embedded_io::Read` trait via `pub use` for I/O abstraction.
- Feature flags:
  - `svg` (enabled by default): SVG conversion (`SVGPlayer`)
  - `tracing` (enabled by default): Log output

#### parser Module (`core/src/parser/`)

Handles binary parsing based on the MS-WMF specification.

- `constants/` - WMF constant definitions (enums, flags)
- `objects/` - WMF object definitions (graphics, structure)
- `records/` - WMF record type definitions and parsing (bitmap, control, drawing, escape, object, state)
- Key types: `MetafileHeader`, `RecordType`, `RecordSize`, `ParseError`, `ReadError`
- Parsing functions: `read`, `read_variable`, `read_*_from_le_bytes` (little-endian)

#### converter Module (`core/src/converter/`)

Converts parsed records into an output format.

- `Player` trait - Interface defining methods to process each WMF record
- `SVGPlayer` - SVG implementation of the `Player` trait (when the `svg` feature is enabled)
- `WMFConverter<B, P>` - Accepts a buffer (`embedded_io::Read`) and a `Player`, then executes conversion
- `Bitmap` - Bitmap conversion helper
- Error types: `ConvertError`, `PlayError`

### wmf-cli

- Single binary consisting of `cli/src/main.rs` only
- Argument parsing with `clap`: `--input`, `--output`, `--quiet`, `--verbose`
- Log control via `tracing-subscriber`

### wmf-wasm

- Consists of `wasm/src/lib.rs` only
- `#![no_std]`, `crate-type = ["cdylib"]`
- Functions exported via `#[wasm_bindgen]`:
  - `convertWmf2Svg(buf: &[u8]) -> Result<String, JsValue>`
  - `setLogLevel(level: &str)`
- Pre-built artifacts in `wasm/dist/`

## Development Environment

### Required Tools

- Rust 1.85.0 (pinned via `rust-toolchain.toml`)
- Rust nightly (for rustfmt and cargo-udeps)
- Docker (for spell-check)

### Optional Tools

- `cargo-machete`, `cargo-udeps` (unused dependency detection)
- `wasm-pack` (WASM build)
- Yarn 1.22.22+ (running WASM demo)

Bulk install of tools:

```sh
make install-tools
```

## Build, Test & Quality Checks

### Key Make Targets

| Command | Description |
| --- | --- |
| `make check` | `cargo check --workspace --all-targets --all-features` |
| `make test` | `cargo test --workspace --all-targets` |
| `make fmt` | `cargo +nightly fmt --all` |
| `make lint` | `cargo clippy --workspace --all-targets --all-features -- --no-deps -D warnings` |
| `make udeps` | `cargo machete` && `cargo +nightly udeps --all-targets` |
| `make spell-check` | Run cSpell via Docker |
| `make ci-suite` | Run all of the above: `spell-check fix fmt lint udeps test` |
| `make wasm` | `wasm-pack build --out-dir dist --target web` |
| `make serve` | Start WASM demo at `localhost:8080` |

### CI (GitHub Actions)

`.github/workflows/ci.yaml` runs the following on PRs and pushes to master:

1. `cargo +nightly fmt` (format check)
2. `cargo clippy` (lint)
3. cSpell (spell check)
4. `cargo test` (unit tests)

## Coding Conventions

### Rust Style

- Edition 2024, MSRV 1.85.0
- Formatted according to `rustfmt.toml` (`cargo +nightly fmt`)
  - Line width: 80 characters (including comments)
  - Imports: grouped by `StdExternalCrate`, merged at `Crate` granularity
  - Uses nightly features (`unstable_features = true`)
- clippy: `all` + `pedantic` enabled at `warn` level
  - Allowed lints: `doc_markdown`, `module_name_repetitions`, `must_use_candidate`, `similar_names`
- Error definitions: uses the `snafu` crate
- Logging: uses the `tracing` crate; can be disabled via feature flag
- WMF record type names follow the specification in `UPPER_SNAKE_CASE` (`non_camel_case_types` and `non_snake_case` are allowed)

### EditorConfig

- UTF-8, LF line endings
- Indentation: 4 spaces (Rust), 2 spaces (HTML, JSON, TOML, YAML, Markdown, Shell), tabs (Makefile)
- Trailing whitespace trimmed (except Markdown)
- Final newline inserted

### Spell Check

- Custom dictionary defined in `.vscode/cspell.json`
- Contains many domain-specific terms from the WMF specification
- When adding new WMF terms, add them to the `words` list in `cspell.json`

## Testing

- Inline tests (`#[cfg(test)]`) in `core/src/parser/mod.rs`
- Integration tests in `core/tests/`
  - `core/tests/mod.rs` as the entry point
  - `core/tests/drawing/` for drawing record tests
- Run tests: `make test` or `cargo test --workspace --all-targets`

## Architecture Notes

### Player Pattern

WMF record processing is abstracted via the `Player` trait.
`WMFConverter` sequentially parses records from a binary stream and calls the corresponding `Player` methods.
To add a new output format, implement the `Player` trait.

### no_std Support

Both `wmf-core` and `wmf-wasm` run under `#![no_std]`.
They use the `alloc` crate (Vec, String, BTreeMap, etc.) instead of `std`.
I/O is abstracted via `embedded_io::Read`.

### Binary Parsing

All data is read in little-endian byte order.
The `read_*_from_le_bytes` function family and the `read_variable` function sequentially read data from a buffer.
Each record type has a `::parse(buf, record_size, record_function)` static method.

## Branching & Releases

- Main branch: `master`
- Dependabot: weekly updates for `cargo` and `github-actions` (targeting `master`)
- Release: `make release version=<tag>` creates and pushes a git tag
