# wmf-rs

Library to parse WMF and convert to SVG (WIP).

## crates

- `wmf-core`
  - `parser` module ... Parsing according to [MS-WMF](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2) specifications.
  - `converter` module ... Converting parsed records to SVG.
- `wmf-cli` ... An example runner for `wmf-core`.

## Requirements

- Rust 1.81.0+ (For Development)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- Yarn 1.22.22+ (To run example)

### Optionals

- Docker
- [cargo-machete](https://github.com/bnjbvr/cargo-machete)
- [cargo-udeps](https://github.com/est31/cargo-udeps)

## Installation

```toml
[dependencies]
wmf-core = { tag = "0.0.1" git = "https://github.com/mythrnr/wmf-rs.git" }
```

## Examples

### Run as CLI

More details, see `wmf-cli` crate.

```bash
$ cargo run --package wmf-cli -- --help
Usage: wmf-cli [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    The WMF file path to convert to SVG
  -o, --output <OUTPUT>  The destination file path to save converted SVG [default: output.svg]
  -q, --quiet            Omit logs except error log
      --verbose          Print debug logs
  -h, --help             Print help
  -V, --version          Print version
```

### Run as WASM on browser

- Run example in http://localhost:8080

```bash
make serve
```

- Enable to set log level by running `setLogLevel(level: "trace" | "debug" | "info" | "warn" | "error")`

```html
<script type="module">
import init, { convertWmf2Svg, setLogLevel } from './wmf_wasm.js';

async function run() {
  await init();
  setLogLevel("debug");

  document.getElementById('input').addEventListener('change', () => {
    const input = document.getElementById("input");
    const files = input.files;

    if (files === null || files.length === 0) {
      return;
    }

    const fileReader = new FileReader();

    fileReader.onload = function (e) {
      const bytes = new Uint8Array(new Uint8Array(e.target.result));
      const output = convertWmf2Svg(bytes);

      document.getElementById("output").innerHTML = output;
    };

    fileReader.readAsArrayBuffer(files[0]);
  });
}

run();
</script>
```
