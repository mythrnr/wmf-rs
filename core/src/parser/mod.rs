mod constants;
mod objects;
mod records;

pub use self::{constants::*, objects::*, records::*};
use crate::imports::*;

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum ParseError {
    #[snafu(display("failed to read buffer: {cause}"))]
    FailedReadBuffer { cause: ReadError },
    /// `cause` is a `Cow<'static, str>` so call sites can pass a
    /// `&'static str` literal without allocating, and still fall
    /// back to a formatted `String` when the message embeds runtime
    /// values.
    #[snafu(display("not supported: {cause}"))]
    NotSupported { cause: Cow<'static, str> },
    #[snafu(display("unexpected enum value: {cause}"))]
    UnexpectedEnumValue { cause: Cow<'static, str> },
    #[snafu(display("unexpected bytes pattern: {cause}"))]
    UnexpectedPattern { cause: Cow<'static, str> },
    /// A fixed-value field carried a value other than the one the
    /// specification mandates (e.g. magic numbers, reserved markers).
    /// `width_bits` records the source field width (8/16/32/64) so the
    /// hex display preserves the leading zeros that the original
    /// `{:#06X}` / `{:#010X}` patterns conveyed.
    #[snafu(display(
        "field `{field}` mismatched: actual {actual:#0w$x}, expected \
         {expected:#0w$x}",
        w = hex_width(*width_bits),
    ))]
    MismatchedField {
        field: &'static str,
        actual: u64,
        expected: u64,
        width_bits: u8,
    },
    /// A field exceeded its allowed maximum, either by specification or
    /// by an internal sanity bound. `width_bits` controls the hex
    /// display width as for `MismatchedField`.
    #[snafu(display(
        "field `{field}` out of range: actual {actual:#0w$x}, max \
         {max:#0w$x}",
        w = hex_width(*width_bits),
    ))]
    FieldOutOfRange {
        field: &'static str,
        actual: u64,
        max: u64,
        width_bits: u8,
    },
    /// A field that the specification requires to be non-negative
    /// carried a negative value (e.g. signed length / count fields).
    /// Negative values lose their bit-width meaning when widened to
    /// `i64`, so this variant displays decimal rather than hex.
    #[snafu(display("field `{field}` must be non-negative: actual {actual}"))]
    FieldNegative { field: &'static str, actual: i64 },
}

/// Maps a bit width (8/16/32/64) to the formatter `width` argument
/// expected by `{:#0w$x}`. The `#` flag emits the `0x` prefix and the
/// `0` flag pads with zeros, so the total width includes those two
/// prefix characters and the underlying hex digits.
const fn hex_width(width_bits: u8) -> usize {
    // hex digits = bits / 4; total width adds 2 for the `0x` prefix.
    (width_bits as usize) / 4 + 2
}

impl ParseError {
    /// Returns `Ok(())` if `actual == expected`, otherwise produces
    /// `MismatchedField` carrying the field name, both values, and the
    /// source bit width so the diagnostic display can keep the
    /// zero-padded hex format that the per-call-site `{:#06X}` /
    /// `{:#010X}` patterns used to provide.
    ///
    /// Generic over `T` so callers pass the field through directly
    /// (`expect_eq("byte_count", byte_count, 0x0004)`); `size_of::<T>()`
    /// then yields the bit width without an extra argument.
    pub(crate) fn expect_eq<T>(
        field: &'static str,
        actual: T,
        expected: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialEq + Into<u64>,
    {
        if actual == expected {
            Ok(())
        } else {
            Err(Self::MismatchedField {
                field,
                actual: actual.into(),
                expected: expected.into(),
                width_bits: bits_of::<T>(),
            })
        }
    }

    /// Returns `Ok(())` if `actual <= max`, otherwise produces
    /// `FieldOutOfRange`. Used for spec-defined upper bounds and our
    /// own DoS guards alike. Generic over `T` for the same reason as
    /// `expect_eq`.
    pub(crate) fn expect_le<T>(
        field: &'static str,
        actual: T,
        max: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialOrd + Into<u64>,
    {
        if actual <= max {
            Ok(())
        } else {
            Err(Self::FieldOutOfRange {
                field,
                actual: actual.into(),
                max: max.into(),
                width_bits: bits_of::<T>(),
            })
        }
    }

    /// Returns `Ok(())` if `actual >= 0`, otherwise produces
    /// `FieldNegative`. Centralizes the recurring "signed length /
    /// count must be non-negative" check. Accepts any signed integer
    /// that widens to `i64`.
    pub(crate) fn expect_non_negative<T>(
        field: &'static str,
        actual: T,
    ) -> Result<(), Self>
    where
        T: Copy + PartialOrd + Default + Into<i64>,
    {
        if actual >= T::default() {
            Ok(())
        } else {
            Err(Self::FieldNegative { field, actual: actual.into() })
        }
    }
}

/// Returns the bit width of `T` (8/16/32/64). Used to thread the source
/// integer width through to the structured `ParseError` variants so
/// their `Display` can emit hex with the appropriate zero padding.
const fn bits_of<T>() -> u8 {
    (core::mem::size_of::<T>() * 8) as u8
}

impl From<ReadError> for ParseError {
    fn from(err: ReadError) -> Self {
        Self::FailedReadBuffer { cause: err }
    }
}

#[derive(Clone, Debug, snafu::prelude::Snafu)]
#[snafu(display("failed to read buffer: {cause}"))]
pub struct ReadError {
    cause: String,
}

impl ReadError {
    pub fn new(err: impl core::fmt::Display) -> Self {
        Self { cause: err.to_string() }
    }
}

#[inline]
pub(in crate::parser) fn read<R: crate::Read, const N: usize>(
    buf: &mut R,
) -> Result<([u8; N], usize), ReadError> {
    let mut buffer = [0u8; N];

    read_exact(buf, &mut buffer)?;

    Ok((buffer, N))
}

pub(crate) fn read_variable<R: crate::Read>(
    buf: &mut R,
    len: usize,
) -> Result<(Vec<u8>, usize), ReadError> {
    // For small payloads `vec![0u8; len]` is the cheapest path: one
    // allocation, one memset, one `read`. For larger payloads (e.g.
    // multi-MB DIB pixel data) the up-front zero-fill becomes a
    // measurable second pass over memory, so we switch over to
    // reading through a stack scratch buffer and `extend_from_slice`
    // into a Vec that started life uninitialized — staying entirely
    // within safe Rust. The threshold matches the chunk size used
    // elsewhere (e.g. `records::consume_remaining_bytes`) for
    // consistency.
    const CHUNK: usize = 4096;

    if len == 0 {
        return Ok((Vec::new(), 0));
    }

    if len <= CHUNK {
        let mut buffer = vec![0u8; len];
        read_exact(buf, &mut buffer)?;
        return Ok((buffer, len));
    }

    let mut buffer = Vec::with_capacity(len);
    let mut scratch = [0u8; CHUNK];
    let mut remaining = len;
    while remaining > 0 {
        let to_read = remaining.min(scratch.len());
        read_exact(buf, &mut scratch[..to_read])?;
        buffer.extend_from_slice(&scratch[..to_read]);
        remaining -= to_read;
    }

    Ok((buffer, len))
}

/// Reads exactly `buffer.len()` bytes from `buf`.
///
/// Defers to `embedded_io::Read::read_exact`, which already loops
/// over short reads and surfaces an `UnexpectedEof` variant — and
/// which a `Read` impl can override with a more efficient bulk
/// read. The hand-rolled loop this replaced was duplicating that
/// default behavior.
fn read_exact<R: crate::Read>(
    buf: &mut R,
    buffer: &mut [u8],
) -> Result<(), ReadError> {
    let total = buffer.len();
    buf.read_exact(buffer).map_err(|err| match err {
        embedded_io::ReadExactError::UnexpectedEof => ReadError::new(format!(
            "expected {total} bytes read, but stream ended early (unexpected \
             end of stream)"
        )),
        embedded_io::ReadExactError::Other(inner) => {
            ReadError::new(format!("{inner:?}"))
        }
    })
}

/// Type-driven dispatch for little-endian fixed-width integer reads.
///
/// Lets generic helpers (e.g. `records::read_field`) pick the right
/// integer width based on the requested type, while keeping the byte-count
/// returned by each impl bound to the type that produced it.
pub(crate) trait ReadLeField: Sized {
    fn read_le<R: crate::Read>(buf: &mut R)
    -> Result<(Self, usize), ReadError>;
}

/// Abstract interface for tracking how many bytes have been consumed from
/// a buffer. Implemented for both `RecordSize` (used by record parsers
/// that have a known frame) and `usize` (used by object/control parsers
/// that just thread a counter through the call graph).
pub(crate) trait ConsumeTracker {
    fn track(&mut self, consumed_bytes: usize);
}

impl ConsumeTracker for usize {
    #[inline]
    fn track(&mut self, consumed_bytes: usize) {
        *self += consumed_bytes;
    }
}

impl ConsumeTracker for crate::parser::RecordSize {
    #[inline]
    fn track(&mut self, consumed_bytes: usize) {
        self.consume(consumed_bytes);
    }
}

impl ReadLeField for i8 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((i8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i16 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((i16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i32 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((i32::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u8 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((u8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u16 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((u16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u32 {
    #[inline]
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((u32::from_le_bytes(bytes), c))
    }
}

/// Converts the given byte slice to a UTF-8 string using the specified
/// character set.
///
/// # Arguments
///
/// - `bytes` - The byte slice to convert.
/// - `charset` - The character set indicating the encoding of the byte slice.
///
/// # Returns
///
/// - On success, returns a UTF-8 string.
/// - On failure to decode, returns a `ParseError`.
///
/// If `SYMBOL_CHARSET` is specified, the function uses the symbol charset table
/// for conversion. Otherwise, it decodes using the provided encoding and
/// removes any null ( `\0` ) characters from the result.
fn bytes_into_utf8(
    bytes: &[u8],
    charset: crate::parser::CharacterSet,
) -> Result<String, crate::parser::ParseError> {
    if charset == crate::parser::CharacterSet::SYMBOL_CHARSET {
        // The symbol-charset lookup table never maps to `\0`, so a second
        // pass to strip null characters would be a no-op allocation.
        Ok(bytes
            .iter()
            .filter_map(|&v| crate::parser::map_symbol_charset(v))
            .collect::<String>())
    } else {
        let encoding: &'static encoding_rs::Encoding = charset.into();
        let (cow, _, had_errors) = encoding.decode(bytes);

        if had_errors {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "Failed to decode string with specified charset".into(),
            });
        }

        // Avoid the unconditional `cow.replace('\0', "")` allocation
        // when the decoded text contains no NUL bytes (the common case
        // for well-formed inputs); otherwise strip them in-place.
        if cow.contains('\0') {
            let mut owned = cow.into_owned();
            owned.retain(|c| c != '\0');
            Ok(owned)
        } else {
            Ok(cow.into_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseError, ReadLeField};

    #[test]
    fn expect_eq_passes_on_match() {
        assert!(ParseError::expect_eq("foo", 0x1234_u16, 0x1234_u16).is_ok());
    }

    #[test]
    fn expect_eq_fails_on_mismatch() {
        let err =
            ParseError::expect_eq("foo", 0x1234_u16, 0x5678_u16).unwrap_err();
        match err {
            ParseError::MismatchedField {
                field,
                actual,
                expected,
                width_bits,
            } => {
                assert_eq!(field, "foo");
                assert_eq!(actual, 0x1234);
                assert_eq!(expected, 0x5678);
                assert_eq!(width_bits, 16);
            }
            _ => panic!("unexpected variant"),
        }
    }

    /// Verifies the source bit width flows through to the variant so
    /// the `Display` impl can pad hex to the original zero-extended
    /// width. A u32 source must be tagged 32 bits, not the receiver's
    /// `u64` upper bound.
    #[test]
    fn expect_eq_records_u32_width() {
        let err = ParseError::expect_eq("magic", 0x0_u32, 0x4346_4D57_u32)
            .unwrap_err();
        let ParseError::MismatchedField { width_bits, .. } = err else {
            panic!("unexpected variant");
        };
        assert_eq!(width_bits, 32);
    }

    /// `Display` must zero-pad both operands to the source bit width.
    /// For u16 inputs that means 4 hex digits + the `0x` prefix.
    #[test]
    fn mismatched_field_display_pads_to_source_width() {
        let err = ParseError::expect_eq("byte_count", 0x0_u16, 0x0004_u16)
            .unwrap_err();
        let s = alloc::string::ToString::to_string(&err);
        assert!(
            s.contains("0x0000") && s.contains("0x0004"),
            "expected zero-padded u16 hex, got: {s}"
        );
    }

    #[test]
    fn expect_le_allows_equal() {
        assert!(ParseError::expect_le("bar", 100_u32, 100_u32).is_ok());
    }

    #[test]
    fn expect_le_rejects_overflow() {
        let err = ParseError::expect_le("bar", 101_u32, 100_u32).unwrap_err();
        match err {
            ParseError::FieldOutOfRange { field, actual, max, width_bits } => {
                assert_eq!(field, "bar");
                assert_eq!(actual, 101);
                assert_eq!(max, 100);
                assert_eq!(width_bits, 32);
            }
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn expect_non_negative_passes_on_zero_and_positive() {
        assert!(ParseError::expect_non_negative("n", 0_i16).is_ok());
        assert!(ParseError::expect_non_negative("n", 42_i32).is_ok());
    }

    #[test]
    fn expect_non_negative_rejects_negative() {
        let err = ParseError::expect_non_negative("n", -1_i16).unwrap_err();
        match err {
            ParseError::FieldNegative { field, actual } => {
                assert_eq!(field, "n");
                assert_eq!(actual, -1);
            }
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn read_i16_from_le_bytes_ok() {
        let data = (-1234_i16).to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = i16::read_le(&mut reader).unwrap();
        assert_eq!(val, -1234);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_u16_from_le_bytes_ok() {
        let data = 0xABCD_u16.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = u16::read_le(&mut reader).unwrap();
        assert_eq!(val, 0xABCD);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_u32_from_le_bytes_ok() {
        let data = 0xDEADBEEF_u32.to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = u32::read_le(&mut reader).unwrap();
        assert_eq!(val, 0xDEADBEEF);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn read_i32_from_le_bytes_ok() {
        let data = (-99999_i32).to_le_bytes();
        let mut reader = &data[..];
        let (val, consumed) = i32::read_le(&mut reader).unwrap();
        assert_eq!(val, -99999);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn read_u8_from_le_bytes_ok() {
        let data = [0xFF];
        let mut reader = &data[..];
        let (val, consumed) = u8::read_le(&mut reader).unwrap();
        assert_eq!(val, 0xFF);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn read_variable_exact() {
        let data = [1, 2, 3, 4, 5];
        let mut reader = &data[..];
        let (result, consumed) = super::read_variable(&mut reader, 5).unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        assert_eq!(consumed, 5);
    }

    #[test]
    fn read_variable_partial_read_fails() {
        let data = [1, 2, 3];
        let mut reader = &data[..];
        assert!(super::read_variable(&mut reader, 5).is_err());
    }

    #[test]
    fn read_variable_zero_length() {
        let mut reader: &[u8] = &[];
        let (data, consumed) = super::read_variable(&mut reader, 0).unwrap();
        assert!(data.is_empty());
        assert_eq!(consumed, 0);
    }

    #[test]
    fn read_fixed_exact_boundary() {
        let data = [0xAA, 0xBB];
        let mut reader = &data[..];
        let (bytes, consumed) = super::read::<&[u8], 2>(&mut reader).unwrap();
        assert_eq!(bytes, [0xAA, 0xBB]);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn read_fixed_empty_buffer() {
        let mut reader: &[u8] = &[];
        assert!(super::read::<&[u8], 2>(&mut reader).is_err());
    }

    #[test]
    fn read_u16_from_empty_buffer() {
        let mut reader: &[u8] = &[];
        assert!(u16::read_le(&mut reader).is_err());
    }

    #[test]
    fn read_u32_from_short_buffer() {
        let data = [0u8; 2];
        let mut reader = &data[..];
        assert!(u32::read_le(&mut reader).is_err());
    }

    #[test]
    fn read_variable_insufficient_data() {
        let data = [0u8; 3];
        let mut reader = &data[..];
        assert!(super::read_variable(&mut reader, 10).is_err());
    }
}
