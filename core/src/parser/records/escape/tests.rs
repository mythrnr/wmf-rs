//! Shared parse tests for `META_ESCAPE` sub-records. Centralizing the
//! escape parser tests in one file (instead of a per-variant module) is
//! a deliberate trade-off: most variants follow the same "fixed
//! `byte_count` + small payload" shape and benefit from data-driven
//! coverage, while the few variants that have variable-length payloads
//! get bespoke tests further down.

use crate::{
    imports::*,
    parser::{
        META_ESCAPE, MetafileEscapes, ParseError,
        records::test_helpers::{build_escape_record, parse_escape_record},
    },
};

/// Verifies that the variant produced by `META_ESCAPE::parse` matches
/// the closure expectation. Closures (rather than `matches!`) keep the
/// table syntax compact while still letting the assertion message
/// identify which row failed.
fn assert_variant<F>(escape_id: u16, byte_count: u16, payload: &[u8], check: F)
where
    F: Fn(&META_ESCAPE) -> bool,
{
    let data = build_escape_record(escape_id, byte_count, payload);
    let parsed = parse_escape_record(&data).unwrap_or_else(|e| {
        panic!("escape {escape_id:#06X} parse failed: {e}")
    });
    assert!(
        check(&parsed),
        "escape {escape_id:#06X} parsed into unexpected variant: {parsed:?}",
    );
}

/// Function pointer type used by the empty-payload coverage table.
/// Hoisted to a `type` alias so clippy's `type_complexity` lint is
/// happy without an `allow` attribute.
type Check = fn(&META_ESCAPE) -> bool;

/// Sanity check that the no-payload escapes (byte_count == 0x0000) all
/// round-trip. Cheap coverage for ~20 variants in one test.
#[test]
fn empty_payload_variants_parse() {
    let cases: &[(u16, Check)] = &[
        (MetafileEscapes::ABORTDOC as u16, |v| {
            matches!(v, META_ESCAPE::ABORTDOC { .. })
        }),
        (MetafileEscapes::BEGIN_PATH as u16, |v| {
            matches!(v, META_ESCAPE::BEGIN_PATH { .. })
        }),
        (MetafileEscapes::CLOSECHANNEL as u16, |v| {
            matches!(v, META_ESCAPE::CLOSECHANNEL { .. })
        }),
        (MetafileEscapes::DOWNLOADFACE as u16, |v| {
            matches!(v, META_ESCAPE::DOWNLOADFACE { .. })
        }),
        (MetafileEscapes::DOWNLOADHEADER as u16, |v| {
            matches!(v, META_ESCAPE::DOWNLOADHEADER { .. })
        }),
        (MetafileEscapes::ENDDOC as u16, |v| {
            matches!(v, META_ESCAPE::ENDDOC { .. })
        }),
        (MetafileEscapes::END_PATH as u16, |v| {
            matches!(v, META_ESCAPE::END_PATH { .. })
        }),
        (MetafileEscapes::EXTTEXTOUT as u16, |v| {
            matches!(v, META_ESCAPE::EXTTEXTOUT { .. })
        }),
        (MetafileEscapes::GETDEVICEUNITS as u16, |v| {
            matches!(v, META_ESCAPE::GETDEVICEUNITS { .. })
        }),
        (MetafileEscapes::GETEXTENDEDTEXTMETRICS as u16, |v| {
            matches!(v, META_ESCAPE::GETEXTENDEDTEXTMETRICS { .. })
        }),
        (MetafileEscapes::GETFACENAME as u16, |v| {
            matches!(v, META_ESCAPE::GETFACENAME { .. })
        }),
        (MetafileEscapes::GETPAIRKERNTABLE as u16, |v| {
            matches!(v, META_ESCAPE::GETPAIRKERNTABLE { .. })
        }),
        (MetafileEscapes::GETPHYSPAGESIZE as u16, |v| {
            matches!(v, META_ESCAPE::GETPHYSPAGESIZE { .. })
        }),
        (MetafileEscapes::GETPRINTINGOFFSET as u16, |v| {
            matches!(v, META_ESCAPE::GETPRINTINGOFFSET { .. })
        }),
        (MetafileEscapes::GETSCALINGFACTOR as u16, |v| {
            matches!(v, META_ESCAPE::GETSCALINGFACTOR { .. })
        }),
        (MetafileEscapes::METAFILE_DRIVER as u16, |v| {
            matches!(v, META_ESCAPE::METAFILE_DRIVER { .. })
        }),
        (MetafileEscapes::NEWFRAME as u16, |v| {
            matches!(v, META_ESCAPE::NEWFRAME { .. })
        }),
        (MetafileEscapes::NEXTBAND as u16, |v| {
            matches!(v, META_ESCAPE::NEXTBAND { .. })
        }),
        (MetafileEscapes::OPENCHANNEL as u16, |v| {
            matches!(v, META_ESCAPE::OPENCHANNEL { .. })
        }),
        (MetafileEscapes::POSTSCRIPT_IGNORE as u16, |v| {
            matches!(v, META_ESCAPE::POSTSCRIPT_IGNORE { .. })
        }),
        (MetafileEscapes::QUERYDIBSUPPORT as u16, |v| {
            matches!(v, META_ESCAPE::QUERYDIBSUPPORT { .. })
        }),
    ];

    for (escape_id, check) in cases {
        assert_variant(*escape_id, 0x0000, &[], check);
    }
}

/// Each empty-payload variant must reject a non-zero `byte_count` with
/// the structured `MismatchedField` so callers can tell why parsing
/// failed without scraping `cause` strings.
#[test]
fn empty_payload_variants_reject_nonzero_byte_count() {
    let escapes = [
        MetafileEscapes::ABORTDOC as u16,
        MetafileEscapes::END_PATH as u16,
        MetafileEscapes::NEWFRAME as u16,
    ];

    for escape_id in escapes {
        let data = build_escape_record(escape_id, 0x0001, &[0x42, 0x00]);
        let err = parse_escape_record(&data).unwrap_err();
        assert!(
            matches!(err, ParseError::MismatchedField {
                field: "byte_count",
                ..
            }),
            "escape {escape_id:#06X}: expected MismatchedField, got {err:?}",
        );
    }
}

/// `byte_count == 0x0002` family: each carries a single 2-byte payload
/// that decodes either as an enum or a raw integer.
#[test]
fn two_byte_payload_variants_parse() {
    // EPSPRINTING: SetEpsPrinting (u16) - any non-zero value indicates
    // the start of EPS printing.
    assert_variant(
        MetafileEscapes::EPSPRINTING as u16,
        0x0002,
        &0x0001_u16.to_le_bytes(),
        |v| matches!(v, META_ESCAPE::EPSPRINTING { set_eps_printing: 1, .. }),
    );

    // QUERYESCSUPPORT: query (MetafileEscapes enum) - use ABORTDOC as
    // a known-valid value.
    assert_variant(
        MetafileEscapes::QUERYESCSUPPORT as u16,
        0x0002,
        &(MetafileEscapes::ABORTDOC as u16).to_le_bytes(),
        |v| matches!(v, META_ESCAPE::QUERYESCSUPPORT { .. }),
    );

    // SETCOPYCOUNT: copy_count (u16).
    assert_variant(
        MetafileEscapes::SETCOPYCOUNT as u16,
        0x0002,
        &5_u16.to_le_bytes(),
        |v| matches!(v, META_ESCAPE::SETCOPYCOUNT { copy_count: 5, .. }),
    );
}

/// `byte_count == 0x0004` family: each carries a single 4-byte payload.
#[test]
fn four_byte_payload_variants_parse() {
    // CLIP_TO_PATH: clip_function (u16, PostScriptClipping enum) +
    // reserved1 (u16). Use SAVE = 0x0000 as the clip function.
    let mut payload = Vec::new();
    payload.extend_from_slice(&0x0000_u16.to_le_bytes());
    payload.extend_from_slice(&0x0000_u16.to_le_bytes());
    assert_variant(
        MetafileEscapes::CLIP_TO_PATH as u16,
        0x0004,
        &payload,
        |v| matches!(v, META_ESCAPE::CLIP_TO_PATH { reserved1: 0, .. }),
    );

    // SETLINECAP: cap (i32 PostScriptCap, FLAT_CAP = 0).
    assert_variant(
        MetafileEscapes::SETLINECAP as u16,
        0x0004,
        &0_i32.to_le_bytes(),
        |v| matches!(v, META_ESCAPE::SETLINECAP { .. }),
    );

    // SETLINEJOIN: join (i32 PostScriptJoin, MITER_JOIN = 0).
    assert_variant(
        MetafileEscapes::SETLINEJOIN as u16,
        0x0004,
        &0_i32.to_le_bytes(),
        |v| matches!(v, META_ESCAPE::SETLINEJOIN { .. }),
    );

    // SETMITERLIMIT: miter_limit (i32).
    assert_variant(
        MetafileEscapes::SETMITERLIMIT as u16,
        0x0004,
        &10_i32.to_le_bytes(),
        |v| matches!(v, META_ESCAPE::SETMITERLIMIT { miter_limit: 10, .. }),
    );
}

/// `CLIP_TO_PATH` validates that `reserved1` is zero; non-zero values
/// must surface as a `MismatchedField` on `reserved1`.
#[test]
fn clip_to_path_rejects_nonzero_reserved1() {
    let mut payload = Vec::new();
    payload.extend_from_slice(&0x0000_u16.to_le_bytes()); // clip_function
    payload.extend_from_slice(&0x0001_u16.to_le_bytes()); // reserved1 != 0
    let data = build_escape_record(
        MetafileEscapes::CLIP_TO_PATH as u16,
        0x0004,
        &payload,
    );
    let err = parse_escape_record(&data).unwrap_err();
    assert!(
        matches!(err, ParseError::MismatchedField { field: "reserved1", .. }),
        "expected MismatchedField on reserved1, got {err:?}",
    );
}

/// `STARTDOC` carries a variable-length doc name bounded by 260 bytes.
#[test]
fn startdoc_parses_with_doc_name() {
    let doc_name = b"hello.ps";
    assert_variant(
        MetafileEscapes::STARTDOC as u16,
        doc_name.len() as u16,
        doc_name,
        |v| matches!(v, META_ESCAPE::STARTDOC { doc_name, .. } if doc_name == b"hello.ps"),
    );
}

/// `STARTDOC`'s spec mandates byte_count < 260; verify the upper bound
/// surfaces as a structured `FieldOutOfRange`.
#[test]
fn startdoc_rejects_oversized_byte_count() {
    let payload = vec![0x41_u8; 300];
    let data =
        build_escape_record(MetafileEscapes::STARTDOC as u16, 300, &payload);
    let err = parse_escape_record(&data).unwrap_err();
    assert!(
        matches!(err, ParseError::FieldOutOfRange { field: "byte_count", .. }),
        "expected FieldOutOfRange on byte_count, got {err:?}",
    );
}

/// `META_ESCAPE_ENHANCED_METAFILE` carries the 30-byte EMF comment
/// header followed by `enhanced_metafile_data_size` raw bytes; build a
/// minimal record with empty EMF data to exercise the validation chain.
#[test]
fn enhanced_metafile_parses_minimal() {
    let mut payload = Vec::new();
    payload.extend_from_slice(&0x4346_4D57_u32.to_le_bytes()); // CommentIdentifier
    payload.extend_from_slice(&0x0000_0001_u32.to_le_bytes()); // CommentType
    payload.extend_from_slice(&0x0001_0000_u32.to_le_bytes()); // Version
    payload.extend_from_slice(&0_u16.to_le_bytes()); // Checksum
    payload.extend_from_slice(&0_u32.to_le_bytes()); // Flags
    payload.extend_from_slice(&1_u32.to_le_bytes()); // CommentRecordCount
    payload.extend_from_slice(&0_u32.to_le_bytes()); // CurrentRecordSize
    payload.extend_from_slice(&0_u32.to_le_bytes()); // RemainingBytes
    payload.extend_from_slice(&0_u32.to_le_bytes()); // EnhancedMetafileDataSize

    assert_variant(
        MetafileEscapes::META_ESCAPE_ENHANCED_METAFILE as u16,
        34,
        &payload,
        |v| matches!(v, META_ESCAPE::META_ESCAPE_ENHANCED_METAFILE { .. }),
    );
}

/// The fixed-value fields in `META_ESCAPE_ENHANCED_METAFILE` must be
/// rejected when corrupted. `comment_identifier` is the most distinctive
/// magic value (`0x4346_4D57` = "WMFC" in little-endian) so it is a
/// good representative for the structured-error path.
#[test]
fn enhanced_metafile_rejects_bad_comment_identifier() {
    let mut payload = Vec::new();
    payload.extend_from_slice(&0xDEAD_BEEF_u32.to_le_bytes()); // wrong magic
    payload.extend_from_slice(&0x0000_0001_u32.to_le_bytes());
    payload.extend_from_slice(&0x0001_0000_u32.to_le_bytes());
    payload.extend_from_slice(&0_u16.to_le_bytes());
    payload.extend_from_slice(&0_u32.to_le_bytes());
    payload.extend_from_slice(&1_u32.to_le_bytes());
    payload.extend_from_slice(&0_u32.to_le_bytes());
    payload.extend_from_slice(&0_u32.to_le_bytes());
    payload.extend_from_slice(&0_u32.to_le_bytes());

    let data = build_escape_record(
        MetafileEscapes::META_ESCAPE_ENHANCED_METAFILE as u16,
        34,
        &payload,
    );
    let err = parse_escape_record(&data).unwrap_err();
    assert!(
        matches!(err, ParseError::MismatchedField {
            field: "comment_identifier",
            ..
        },),
        "expected MismatchedField on comment_identifier, got {err:?}",
    );
}

/// Variable-payload escapes (PASSTHROUGH/POSTSCRIPT_*) accept whatever
/// byte_count the caller supplies, so the round-trip just verifies the
/// data slice ends up in the right field.
#[test]
fn passthrough_round_trips_payload() {
    let bytes = b"raw data";
    assert_variant(
        MetafileEscapes::PASSTHROUGH as u16,
        bytes.len() as u16,
        bytes,
        |v| matches!(v, META_ESCAPE::PASSTHROUGH { data, .. } if data == b"raw data"),
    );
}

#[test]
fn postscript_data_round_trips_payload() {
    let bytes = b"showpage\n";
    assert_variant(
        MetafileEscapes::POSTSCRIPT_DATA as u16,
        bytes.len() as u16,
        bytes,
        |v| matches!(v, META_ESCAPE::POSTSCRIPT_DATA { data, .. } if data == b"showpage\n"),
    );
}

/// The dispatcher's lower-byte check surfaces as a `MismatchedField` on
/// `record_function` (low byte). Verifying it once protects every
/// escape variant from accidental dispatcher regressions.
#[test]
fn parse_rejects_record_function_mismatch() {
    // Manually build a record whose record_function lower byte is *not*
    // META_ESCAPE (0x26) so check_lower_byte_matches fires.
    let mut data = Vec::new();
    data.extend_from_slice(&5_u32.to_le_bytes()); // word_count
    data.extend_from_slice(&0x06FF_u16.to_le_bytes()); // wrong record_function
    data.extend_from_slice(&(MetafileEscapes::ABORTDOC as u16).to_le_bytes());
    data.extend_from_slice(&0_u16.to_le_bytes());
    let err = parse_escape_record(&data).unwrap_err();
    assert!(
        matches!(err, ParseError::MismatchedField {
            field: "record_function (low byte)",
            ..
        },),
        "expected lower-byte MismatchedField, got {err:?}",
    );
}
