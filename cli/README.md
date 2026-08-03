# wmf-cli

A command-line tool for converting
[WMF (Windows Metafile)](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2)
binaries to SVG, built on
[`wmf-core`](https://github.com/mythrnr/wmf-rs/tree/master/core).

> **Note:** This project is a work in progress. Some WMF records are not yet
> fully implemented.

## Installation

Install from the repository:

```sh
cargo install --git https://github.com/mythrnr/wmf-rs wmf-cli
```

## Usage

```sh
wmf-cli --input sample.wmf --output out.svg
```

```text
Usage: wmf-cli [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    The WMF file path to convert to SVG
  -o, --output <OUTPUT>  The destination file path to save converted SVG [default: output.svg]
  -q, --quiet            Omit logs except error log
      --verbose          Print debug logs
  -h, --help             Print help
  -V, --version          Print version
```

## License

This project is licensed under the
[MIT License](https://github.com/mythrnr/wmf-rs/blob/master/LICENSE).
