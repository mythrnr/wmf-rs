# wmf-core

A Rust library for parsing
[WMF (Windows Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2)
binaries and converting them to SVG.

> **Note:** This project is a work in progress. Some WMF records are not yet
> fully implemented.

## Features

- Parses the WMF binary format according to the MS-WMF specification
- Converts WMF records to SVG output
- `no_std` compatible (uses `alloc`)
- Extensible conversion via the `Player` trait

## Installation

```sh
cargo add wmf-core
```

### Feature Flags

| Feature   | Default | Description                                   |
| --------- | ------- | --------------------------------------------- |
| `svg`     | Yes     | Enables SVG conversion (`SVGPlayer`)          |
| `tracing` | Yes     | Enables log output via the `tracing` crate    |

To use with minimal dependencies:

```sh
cargo add wmf-core --no-default-features
```

## Usage

```rust
use std::fs;

fn main() {
    let wmf_data = fs::read("input.wmf").expect("failed to read file");

    match wmf_core::converter::convert_to_svg(wmf_data.as_slice()) {
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

Pass the implementation to `wmf_core::converter::convert`:

```rust
let output = wmf_core::converter::convert(
    wmf_data.as_slice(),
    MyPlayer { /* ... */ },
)
.expect("failed to convert");
```

## License

This project is licensed under the
[MIT License](https://github.com/mythrnr/wmf-rs/blob/master/LICENSE).

Portions of the API documentation are adapted from the
[MS-WMF Open Specifications documentation](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2),
© Microsoft Corporation, and are used under the Intellectual Property Rights
Notice for Open Specifications Documentation. The MS-WMF specification is
covered by the
[Microsoft Open Specification Promise](https://go.microsoft.com/fwlink/?LinkId=214445).
