//! Implementation of the definitions in Section 2.3 of the WMF specifications.

mod bitmap;
mod control;
mod drawing;
mod escape;
mod object;
mod state;

pub use self::{
    bitmap::*, control::*, drawing::*, escape::*, object::*, state::*,
};

// ---------------------------------------------------------------------------
// RecordSize: the central record-frame primitive
// ---------------------------------------------------------------------------

/// A 32-bit unsigned integer that defines the number of 16-bit WORD structures,
/// defined in [MS-DTYP] section 2.2.61, in the record.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct RecordSize {
    /// Total record length in 16-bit WORDs, taken verbatim from the
    /// `RecordSize` field of the record header.
    size_words: u32,
    /// Number of bytes already consumed from the record payload, used to
    /// detect overruns and to skip the trailing reserved area.
    consumed_bytes: usize,
}

impl RecordSize {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<Self, crate::parser::ParseError> {
        /// Maximum allowed record size in WORDs.
        /// 32 MW (64 MB) is far beyond any practical WMF record and guards
        /// against crafted inputs that would cause excessive resource usage.
        const MAX_RECORD_WORDS: u32 = 32 * 1024 * 1024;

        let (v, c) = <u32 as crate::parser::ReadLeField>::read_le(buf)?;

        // Minimum record is 3 WORDs (RecordSize: 2 + RecordFunction: 1).
        if v < 3 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "record size {v:#010X} is smaller than minimum header \
                     size (0x00000003)",
                )
                .into(),
            });
        }

        crate::parser::ParseError::expect_le(
            "record_size (words)",
            v,
            MAX_RECORD_WORDS,
        )?;

        Ok(Self { size_words: v, consumed_bytes: c })
    }

    /// Constructs a `RecordSize` from a raw word count without going
    /// through `parse`'s validation. Intended only for synthesizing
    /// records in unit/integration tests; production code MUST go
    /// through `parse` so `MAX_RECORD_WORDS` and minimum-size checks
    /// stay in effect.
    #[doc(hidden)]
    pub fn from_raw(words: u32) -> Self {
        Self { size_words: words, consumed_bytes: 0 }
    }

    #[inline]
    pub fn word_size(&self) -> usize {
        self.size_words as usize
    }

    #[inline]
    pub fn byte_count(&self) -> usize {
        // `parse` already rejects `size_words` above
        // `MAX_RECORD_WORDS` (32 MW), so the doubled value cannot
        // overflow `usize` on any supported platform; a plain
        // multiplication keeps this hot getter branch-free.
        (self.size_words as usize) * 2
    }

    #[inline]
    pub fn consumed_bytes(&self) -> usize {
        self.consumed_bytes
    }

    #[inline]
    pub fn consume(&mut self, consumed_bytes: usize) {
        self.consumed_bytes += consumed_bytes;
    }

    #[inline]
    pub fn remaining_bytes(&self) -> usize {
        self.byte_count().saturating_sub(self.consumed_bytes)
    }

    /// Returns true if consumed_bytes has exceeded byte_count,
    /// indicating a malformed record or a parser bug.
    #[inline]
    pub fn is_overrun(&self) -> bool {
        self.consumed_bytes > self.byte_count()
    }

    #[inline]
    pub fn remaining(&self) -> bool {
        !self.is_overrun() && self.remaining_bytes() > 0
    }
}

impl From<RecordSize> for u32 {
    fn from(v: RecordSize) -> Self {
        v.size_words
    }
}

impl core::fmt::Display for RecordSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#010X}", self.size_words)
    }
}

impl core::fmt::Debug for RecordSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "RecordSize(size: {:#010X}, consumed_bytes: {})",
            self.size_words, self.consumed_bytes
        )
    }
}

// ---------------------------------------------------------------------------
// Display-only helpers used by the per-record `tracing::instrument`
// ---------------------------------------------------------------------------

/// `Display`-only wrapper around a `u16` that emits `{:#06X}`.
///
/// `tracing::instrument(fields(record_function = %...))` previously
/// passed `format!("{record_function:#06X}")`, which allocated a
/// `String` on every parse just to feed `tracing`'s `Display`
/// recorder. Wrapping the raw `u16` in this newtype lets the macro
/// hand `tracing` a `Display` impl directly so the formatting
/// happens lazily inside the subscriber, without an intermediate
/// heap allocation. The visible output (`0xNNNN`) is preserved.
#[doc(hidden)]
pub struct HexU16(pub u16);

impl core::fmt::Display for HexU16 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#06X}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Field-read helpers shared by every record / object parser
// ---------------------------------------------------------------------------

/// Read a fixed-width little-endian integer field, advance the `tracker`
/// by the number of bytes consumed, and return the value.
///
/// Collapses the recurring three-step pattern in record parsers
/// (call `read_<ty>_from_le_bytes`, capture both value and byte count,
/// then advance the byte counter) so the byte-count bookkeeping cannot
/// drift from the actual read. The tracker can be either a `RecordSize`
/// (record parsers) or a plain `usize` counter (object/control parsers).
/// The output type is selected via type inference from the binding,
/// e.g. `let v = read_field(...)?;`.
#[inline]
pub(crate) fn read_field<R, T>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
) -> Result<T, crate::parser::ParseError>
where
    R: crate::Read,
    T: crate::parser::ReadLeField,
{
    let (v, c) = T::read_le(buf)?;
    tracker.track(c);
    Ok(v)
}

/// Run a composite parser that returns `(value, consumed_bytes)` and
/// advance the `tracker` accordingly. Used for object parsers
/// (e.g. `ColorRef::parse`) that already follow the `(T, usize)`
/// convention but cannot satisfy the `ReadLeField` bound.
#[inline]
pub(in crate::parser) fn read_with<R, T, F>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
    parser: F,
) -> Result<T, crate::parser::ParseError>
where
    R: crate::Read,
    F: FnOnce(&mut R) -> Result<(T, usize), crate::parser::ParseError>,
{
    let (v, c) = parser(buf)?;
    tracker.track(c);
    Ok(v)
}

/// Read a variable-length byte buffer of `len` bytes and advance the
/// `tracker` accordingly.
#[inline]
pub(in crate::parser) fn read_bytes_field<R>(
    buf: &mut R,
    tracker: &mut impl crate::parser::ConsumeTracker,
    len: usize,
) -> Result<crate::imports::Vec<u8>, crate::parser::ParseError>
where
    R: crate::Read,
{
    let (v, c) = crate::parser::read_variable(buf, len)?;
    tracker.track(c);
    Ok(v)
}

/// Check lower byte MUST match the lower byte of the RecordType Enumeration
/// table value.
fn check_lower_byte_matches(
    record_function: u16,
    record_type: crate::parser::RecordType,
) -> Result<(), crate::parser::ParseError> {
    crate::parser::ParseError::expect_eq(
        "record_function (low byte)",
        record_function & 0x00FF,
        (record_type as u16) & 0x00FF,
    )
}

/// Drains and discards any unread payload bytes for a record.
///
/// All call sites discard the bytes anyway, so the function returns
/// `()` on success rather than handing back an empty `Vec` and a
/// byte count that the previous signature implied carried a payload.
fn consume_remaining_bytes<R: crate::Read>(
    buf: &mut R,
    record_size: crate::parser::RecordSize,
) -> Result<(), crate::parser::ParseError> {
    crate::parser::ParseError::expect_le(
        "consumed_bytes",
        record_size.consumed_bytes() as u64,
        record_size.byte_count() as u64,
    )?;

    let remaining = record_size.remaining_bytes();
    if remaining == 0 {
        return Ok(());
    }

    // Discard remaining bytes in fixed-size chunks to avoid
    // large allocations from crafted RecordSize values.
    let mut discarded = 0;
    let mut chunk = [0u8; 4096];

    while discarded < remaining {
        let to_read = core::cmp::min(remaining - discarded, chunk.len());

        crate::parser::read_exact(buf, &mut chunk[..to_read])?;
        discarded += to_read;
    }

    Ok(())
}

/// Test helpers for record parsing tests.
#[cfg(test)]
pub(crate) mod test_helpers {
    use crate::imports::*;

    /// Build a record binary payload.
    pub fn build_record(
        word_count: u32,
        record_function: u16,
        payload: &[u8],
    ) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&word_count.to_le_bytes());
        data.extend_from_slice(&record_function.to_le_bytes());
        data.extend_from_slice(payload);
        data
    }

    /// Parse RecordSize and record_function from a pre-built buffer,
    /// consuming both (matching the converter's actual flow).
    pub fn parse_record_header(data: &[u8]) -> (super::RecordSize, u16, &[u8]) {
        let mut reader = data;
        let mut record_size = super::RecordSize::parse(&mut reader).unwrap();
        let record_function: u16 =
            super::read_field(&mut reader, &mut record_size).unwrap();
        (record_size, record_function, reader)
    }

    /// Build a complete META_ESCAPE record from the escape sub-type,
    /// byte_count, and payload bytes. Computes the enclosing
    /// `record_size` automatically so callers only need to supply the
    /// escape-specific bytes. Pads odd-length payloads with a single
    /// zero byte so the resulting record stays word-aligned, mirroring
    /// real-world WMF encoders.
    pub fn build_escape_record(
        escape_id: u16,
        byte_count: u16,
        payload: &[u8],
    ) -> Vec<u8> {
        let mut inner = Vec::new();
        inner.extend_from_slice(&escape_id.to_le_bytes());
        inner.extend_from_slice(&byte_count.to_le_bytes());
        inner.extend_from_slice(payload);
        // RecordSize (4 bytes) + record_function (2 bytes) + inner
        // payload. Pad to an even length so the WORD-based size below
        // can swallow the whole record.
        let total_len = 4 + 2 + inner.len();
        if total_len % 2 != 0 {
            inner.push(0);
        }
        let word_count = ((4 + 2 + inner.len()) / 2) as u32;
        build_record(
            word_count,
            crate::parser::RecordType::META_ESCAPE as u16,
            &inner,
        )
    }

    /// Drive `META_ESCAPE::parse` from a fully built record buffer.
    /// Centralizes the header read / dispatch so test cases stay
    /// focused on payload semantics.
    pub fn parse_escape_record(
        data: &[u8],
    ) -> Result<crate::parser::META_ESCAPE, crate::parser::ParseError> {
        let (rs, rf, mut reader) = parse_record_header(data);
        crate::parser::META_ESCAPE::parse(&mut reader, rs, rf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    #[test]
    fn record_size_remaining_bytes_saturates() {
        let data = 3_u32.to_le_bytes();
        let mut reader = &data[..];
        let mut rs = RecordSize::parse(&mut reader).unwrap();
        assert_eq!(rs.byte_count(), 6);
        rs.consume(6);
        assert_eq!(rs.remaining_bytes(), 0);
        // Over-consume: remaining_bytes saturates to 0.
        rs.consume(10);
        assert_eq!(rs.remaining_bytes(), 0);
    }

    #[test]
    fn consume_remaining_bytes_detects_overrun() {
        let data = 3_u32.to_le_bytes(); // byte_count = 6
        let mut reader = &data[..];
        let mut rs = RecordSize::parse(&mut reader).unwrap();
        // Simulate consuming more than byte_count.
        rs.consume(100);
        let mut empty: &[u8] = &[];
        assert!(
            super::consume_remaining_bytes(&mut empty, rs).is_err(),
            "overrun should be detected as ParseError"
        );
    }

    #[test]
    fn parse_rejects_oversized_word_count() {
        let data = u32::MAX.to_le_bytes();
        let mut reader = &data[..];
        assert!(
            RecordSize::parse(&mut reader).is_err(),
            "oversized RecordSize should be rejected"
        );
    }

    #[test]
    fn record_size_remaining_returns_correct_value() {
        let data = 5_u32.to_le_bytes(); // 10 bytes total
        let mut reader = &data[..];
        let mut rs = RecordSize::parse(&mut reader).unwrap();
        assert_eq!(rs.consumed_bytes(), 4);
        rs.consume(2); // record_function
        assert_eq!(rs.remaining_bytes(), 4);
        rs.consume(4);
        assert!(!rs.remaining());
    }

    #[test]
    fn record_function_lower_byte_mismatch() {
        let payload = [0u8; 8];
        let data = test_helpers::build_record(7, 0x04FF, &payload);
        let (rs, rf, mut reader) = test_helpers::parse_record_header(&data);
        assert!(
            crate::parser::META_RECTANGLE::parse(&mut reader, rs, rf).is_err(),
            "lower byte mismatch should be rejected"
        );
    }

    /// Verifies `read_field` dispatches to the correct integer width
    /// per binding type and advances the supplied `RecordSize` by exactly
    /// the number of bytes consumed.
    #[test]
    fn read_field_reads_and_consumes() -> Result<(), crate::parser::ParseError>
    {
        let mut data = Vec::new();
        data.extend_from_slice(&0x12_u8.to_le_bytes());
        data.extend_from_slice(&(-2_i8).to_le_bytes());
        data.extend_from_slice(&0xABCD_u16.to_le_bytes());
        data.extend_from_slice(&(-1234_i16).to_le_bytes());
        data.extend_from_slice(&0xDEADBEEF_u32.to_le_bytes());
        data.extend_from_slice(&(-99999_i32).to_le_bytes());

        let mut reader: &[u8] = &data;
        let mut rs = RecordSize::from_raw(100);

        let read_u8: u8 = read_field(&mut reader, &mut rs)?;
        let read_i8: i8 = read_field(&mut reader, &mut rs)?;
        let read_u16: u16 = read_field(&mut reader, &mut rs)?;
        let read_i16: i16 = read_field(&mut reader, &mut rs)?;
        let read_u32: u32 = read_field(&mut reader, &mut rs)?;
        let read_i32: i32 = read_field(&mut reader, &mut rs)?;

        assert_eq!(read_u8, 0x12_u8);
        assert_eq!(read_i8, -2_i8);
        assert_eq!(read_u16, 0xABCD_u16);
        assert_eq!(read_i16, -1234_i16);
        assert_eq!(read_u32, 0xDEADBEEF_u32);
        assert_eq!(read_i32, -99999_i32);
        assert_eq!(rs.consumed_bytes(), 1 + 1 + 2 + 2 + 4 + 4);

        Ok(())
    }
}
