# wmf-wasm

WebAssembly bindings for converting
[WMF (Windows Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2)
binaries to SVG in the browser, built on
[`wmf-core`](https://github.com/mythrnr/wmf-rs/tree/master/core) with
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

> **Note:** This project is a work in progress. Some WMF records are not yet
> fully implemented.

## Feature Flags

| Feature                   | Default | Description                                        |
| ------------------------- | ------- | -------------------------------------------------- |
| `console_error_panic_hook` | Yes    | Reports panics via `console.error`                 |
| `tracing`                 | Yes     | Enables `setLogLevel` and browser-console logging  |

## Installation

Pre-built artifacts (`wmf_wasm_bg.wasm`, `wmf_wasm.js`, `wmf_wasm.d.ts`) are
attached to each release on the
[GitHub Releases](https://github.com/mythrnr/wmf-rs/releases) page in two
variants:

- `wmf-wasm-<version>.tar.gz` — full build with `tracing` enabled; pair with
  `setLogLevel` for browser-console logging.
- `wmf-wasm-minimal-<version>.tar.gz` — built without the `tracing` feature.
  `setLogLevel` becomes a no-op, but the bundle is noticeably smaller because
  the `tracing-wasm` dependency is dropped entirely.

To build from source instead:

```sh
make wasm          # full build -> wasm/dist
make wasm-minimal  # without tracing -> wasm/dist-minimal
```

## Usage

```html
<script type="module">
import init, { convertWmf2Svg, setLogLevel } from "./wmf_wasm.js";

async function run() {
  await init();
  setLogLevel("info");

  const bytes = new Uint8Array(await file.arrayBuffer());
  const svg = convertWmf2Svg(bytes);

  document.getElementById("output").innerHTML = svg;
}

run();
</script>
```

To build and run the demo locally:

```sh
make serve
# Open http://localhost:8080
```

## API

- `convertWmf2Svg(buf: Uint8Array): string` - Converts WMF binary data to an
  SVG string.
- `setLogLevel(level: "trace" | "debug" | "info" | "warn" | "error")` - Sets
  the log level (default: `info`).
  - **Note:** `trace` and `debug` levels are very slow to execute.
  - If the `tracing` feature is disabled, `setLogLevel` has no effect.

## License

This project is licensed under the
[MIT License](https://github.com/mythrnr/wmf-rs/blob/master/LICENSE).
