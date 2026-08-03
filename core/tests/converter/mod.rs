use wmf_core::converter::{ConvertError, PlayError, Player};

/// Builds a minimal `META_HEADER` (memory metafile, version 3) so each
/// test case can focus on the records that follow it.
fn wmf_header() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&0x0001_u16.to_le_bytes()); // Type: memory
    data.extend_from_slice(&9_u16.to_le_bytes()); // HeaderSize in words
    data.extend_from_slice(&0x0300_u16.to_le_bytes()); // Version
    data.extend_from_slice(&0_u16.to_le_bytes()); // SizeLow
    data.extend_from_slice(&0_u16.to_le_bytes()); // SizeHigh
    data.extend_from_slice(&0_u16.to_le_bytes()); // NumberOfObjects
    data.extend_from_slice(&0_u32.to_le_bytes()); // MaxRecord
    data.extend_from_slice(&0_u16.to_le_bytes()); // NumberOfMembers
    data
}

/// Encodes a record as `RecordSize` + `RecordFunction` + params, with
/// the size derived from the parameter count so cases stay consistent.
fn record(function: u16, params: &[i16]) -> Vec<u8> {
    let words = 3 + u32::try_from(params.len()).expect("too many params");
    let mut data = Vec::new();
    data.extend_from_slice(&words.to_le_bytes());
    data.extend_from_slice(&function.to_le_bytes());

    for param in params {
        data.extend_from_slice(&param.to_le_bytes());
    }

    data
}

fn wmf_binary(records: &[Vec<u8>]) -> Vec<u8> {
    let mut data = wmf_header();

    for r in records {
        data.extend_from_slice(r);
    }

    data
}

const META_EOF: u16 = 0x0000;
const META_SETWINDOWEXT: u16 = 0x020C;
const META_RECTANGLE: u16 = 0x041B;

#[test]
fn convert_to_svg_renders_records() {
    struct TestCase {
        desc: &'static str,
        records: Vec<Vec<u8>>,
        expected_svg: &'static str,
    }

    let cases = [
        TestCase {
            desc: "Empty metafile produces an empty SVG document",
            records: vec![record(META_EOF, &[])],
            expected_svg: r#"<svg viewBox="0 0 0 0" xmlns="http://www.w3.org/2000/svg"></svg>"#,
        },
        TestCase {
            desc: "Window extent becomes the viewBox",
            records: vec![
                // META_SETWINDOWEXT carries Y then X on the wire.
                record(META_SETWINDOWEXT, &[200, 320]),
                record(META_EOF, &[]),
            ],
            expected_svg: r#"<svg viewBox="0 0 320 200" xmlns="http://www.w3.org/2000/svg"></svg>"#,
        },
        TestCase {
            desc: "Rectangle is rendered with default pen and brush",
            records: vec![
                record(META_SETWINDOWEXT, &[200, 200]),
                // META_RECTANGLE carries Bottom, Right, Top, Left.
                record(META_RECTANGLE, &[120, 150, 20, 10]),
                record(META_EOF, &[]),
            ],
            expected_svg: r##"<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg"><rect fill="none" fill-rule="evenodd" height="100" id="elem2" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1" width="140" x="10" y="20"></rect></svg>"##,
        },
    ];

    for (i, case) in cases.iter().enumerate() {
        let data = wmf_binary(&case.records);
        let svg = wmf_core::converter::convert_to_svg(data.as_slice())
            .unwrap_or_else(|err| {
                panic!("case {i}: {}: conversion failed: {err}", case.desc)
            });
        let svg_str = String::from_utf8(svg).expect("SVG output is not UTF-8");

        assert_eq!(
            svg_str.trim(),
            case.expected_svg.trim(),
            "case {i}: {}: SVG output does not match expected",
            case.desc,
        );
    }
}

#[test]
fn convert_to_svg_rejects_broken_input() {
    struct TestCase {
        desc: &'static str,
        data: Vec<u8>,
    }

    let cases = [
        TestCase { desc: "Empty input", data: Vec::new() },
        TestCase {
            desc: "Truncated metafile header",
            data: wmf_header()[..8].to_vec(),
        },
        TestCase { desc: "Header without any record", data: wmf_binary(&[]) },
        TestCase {
            desc: "Record size smaller than the record header",
            data: {
                let mut data = wmf_header();
                data.extend_from_slice(&0_u32.to_le_bytes());
                data.extend_from_slice(&META_EOF.to_le_bytes());
                data
            },
        },
        TestCase {
            desc: "Unknown record function",
            data: wmf_binary(&[record(0x7FFF, &[]), record(META_EOF, &[])]),
        },
        TestCase {
            desc: "Record payload shorter than the declared size",
            data: {
                let mut data = wmf_header();
                // Declares 5 words but provides no parameters.
                data.extend_from_slice(&5_u32.to_le_bytes());
                data.extend_from_slice(&META_SETWINDOWEXT.to_le_bytes());
                data
            },
        },
    ];

    for (i, case) in cases.iter().enumerate() {
        let result = wmf_core::converter::convert_to_svg(case.data.as_slice());

        assert!(
            matches!(result, Err(ConvertError::ParseError { .. })),
            "case {i}: {}: expected ParseError, got {result:?}",
            case.desc,
        );
    }
}

/// A player that ignores every record, to prove that `convert` drives
/// any `Player` implementation and that unhandled records fall back to
/// the trait's default no-op handlers.
struct NullPlayer;

impl Player for NullPlayer {
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        Ok(b"custom output".to_vec())
    }

    fn header(
        self,
        _record_number: usize,
        _header: wmf_core::parser::MetafileHeader,
    ) -> Result<Self, PlayError> {
        Ok(self)
    }
}

#[test]
fn convert_drives_custom_player() {
    let data = wmf_binary(&[
        record(META_SETWINDOWEXT, &[200, 200]),
        record(META_RECTANGLE, &[120, 150, 20, 10]),
        record(META_EOF, &[]),
    ]);

    let output = wmf_core::converter::convert(data.as_slice(), NullPlayer)
        .expect("conversion failed");

    assert_eq!(output, b"custom output");
}
