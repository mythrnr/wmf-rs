# wmf-rs

A Rust library for parsing [WMF (Windows Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2) binaries and converting them to SVG.

> **Note:** This project is a work in progress. Some WMF records are not yet fully implemented.

## Features

- Parses WMF binary format according to the MS-WMF specification
- Converts WMF records to SVG output
- `no_std` compatible (uses `alloc`)
- Works in WebAssembly environments via `wmf-wasm`
- Extensible conversion via the `Player` trait

## Installation

Add `wmf-core` to your `Cargo.toml`:

```toml
[dependencies]
wmf-core = { git = "https://github.com/mythrnr/wmf-rs.git", tag = "0.0.28", package = "wmf-core" }
```

### Feature Flags

| Feature | Default | Description |
| --- | --- | --- |
| `svg` | Yes | Enables SVG conversion (`SVGPlayer`) |
| `tracing` | Yes | Enables log output via the `tracing` crate |

To use with minimal dependencies:

```toml
[dependencies]
wmf-core = { git = "https://github.com/mythrnr/wmf-rs.git", tag = "0.0.28", package = "wmf-core", default-features = false }
```

## Usage

### As a Rust Library

```rust
use std::fs;

fn main() {
    let wmf_data = fs::read("input.wmf").expect("failed to read file");

    let player = wmf_core::converter::SVGPlayer::new();
    let converter = wmf_core::converter::WMFConverter::new(
        wmf_data.as_slice(),
        player,
    );

    match converter.run() {
        Ok(svg_bytes) => {
            let svg = String::from_utf8_lossy(&svg_bytes);
            println!("{svg}");
        }
        Err(err) => {
            eprintln!("conversion failed: {err}");
        }
    }
}
```

### Custom Player

The conversion process is abstracted through the `Player` trait.
You can implement your own `Player` to produce output formats other than SVG.
Only `generate` and `header` are required; every other record handler has a
default implementation that skips the record, so override only the records
your output format supports:

```rust
use wmf_core::converter::{Player, PlayError};
use wmf_core::parser::*;

struct MyPlayer { /* ... */ }

impl Player for MyPlayer {
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        // Produce your output format here
        todo!()
    }

    fn header(
        self,
        record_number: usize,
        header: MetafileHeader,
    ) -> Result<Self, PlayError> {
        // Set up the canvas from the metafile header
        todo!()
    }

    // Override only the record handlers you support...
    // See `wmf_core::converter::Player` for the full list.
    fn rectangle(
        self,
        record_number: usize,
        record: META_RECTANGLE,
    ) -> Result<Self, PlayError> {
        todo!()
    }
}
```

### As a CLI Tool

The `wmf-cli` crate provides a command-line converter:

```sh
cargo run --package wmf-cli -- --input sample.wmf --output out.svg
```

```sh
Usage: wmf-cli [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    The WMF file path to convert to SVG
  -o, --output <OUTPUT>  The destination file path to save converted SVG [default: output.svg]
  -q, --quiet            Omit logs except error log
      --verbose          Print debug logs
  -h, --help             Print help
  -V, --version          Print version
```

### As WASM in the Browser

The `wmf-wasm` crate provides WebAssembly bindings built with `wasm-pack`.

```html
<script type="module">
import init, { convertWmf2Svg, setLogLevel } from "./wmf_wasm.js";

async function run() {
  await init();
  setLogLevel("info");

  document.getElementById("input").addEventListener("change", () => {
    const input = document.getElementById("input");
    const files = input.files;

    if (files === null || files.length === 0) {
      return;
    }

    const fileReader = new FileReader();

    fileReader.onload = function (e) {
      const bytes = new Uint8Array(e.target.result);
      const svg = convertWmf2Svg(bytes);

      document.getElementById("output").innerHTML = svg;
    };

    fileReader.readAsArrayBuffer(files[0]);
  });
}

run();
</script>
```

To build and run the WASM demo locally:

```sh
make serve
# Open http://localhost:8080
```

Pre-built artifacts (`wmf_wasm_bg.wasm`, `wmf_wasm.js`, `wmf_wasm.d.ts`) are
attached to each release on the
[GitHub Releases](https://github.com/mythrnr/wmf-rs/releases) page in two
variants:

- `wmf-wasm-<version>.tar.gz` — full build with `tracing` enabled; pair with
  `setLogLevel` for browser-console logging.
- `wmf-wasm-minimal-<version>.tar.gz` — built without the `tracing` feature.
  `setLogLevel` becomes a no-op, but the bundle is noticeably smaller because
  the `tracing-wasm` dependency is dropped entirely.

#### WASM API

- `convertWmf2Svg(buf: Uint8Array): string` - Converts WMF binary data to an SVG string.
- `setLogLevel(level: "trace" | "debug" | "info" | "warn" | "error")` - Sets the log level (default: `info`).
  - **Note:** `trace` and `debug` levels are very slow to execute.
  - If the `tracing` feature is disabled, `setLogLevel` has no effect.

## Crate Overview

| Crate | Description |
| --- | --- |
| `wmf-core` | Core library: WMF parser and SVG converter (`no_std`) |
| `wmf-cli` | CLI tool for WMF to SVG conversion |
| `wmf-wasm` | WASM bindings for browser usage (`no_std`) |

## Requirements (for Development)

- Rust 1.88.0+
- Rust nightly toolchain (for `rustfmt` and `cargo-udeps`)
- Docker (for spell-check)
- [wasm-pack](https://github.com/rustwasm/wasm-pack) (for WASM builds)
- Yarn 1.22.22+ (to run the WASM demo)

Optional tools can be installed with:

```sh
make install-tools
```

## Releasing

Direct pushes to `master` are forbidden, so a release is driven by a
version-bump PR:

1. Bump the version:

   ```sh
   make release version=<x.y.z>
   ```

   This updates `[workspace.package].version` and dependent version
   requirements via `cargo release version`, then refreshes `Cargo.lock`.
   Nothing is committed, tagged, or pushed.

2. Commit the result and open a PR. Merging it to `master` is the release
   trigger.

3. On the merge, `tag-release.yaml` creates the matching `<version>` git
   tag and invokes the release workflow, which verifies that the version
   equals the workspace version and publishes the WASM bundles as GitHub
   Releases assets.

To re-run a release whose tag already exists, dispatch the "Release"
workflow manually from the Actions tab with the version as input.

## License

This project is licensed under the [MIT License](LICENSE).

Portions of the API documentation are adapted from the
[MS-WMF Open Specifications documentation](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2),
© Microsoft Corporation, and are used under the Intellectual Property Rights
Notice for Open Specifications Documentation. The MS-WMF specification is
covered by the
[Microsoft Open Specification Promise](https://go.microsoft.com/fwlink/?LinkId=214445).
