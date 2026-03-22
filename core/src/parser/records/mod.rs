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

/// Check lower byte MUST match the lower byte of the RecordType Enumeration
/// table value.
fn check_lower_byte_matches(
    record_function: u16,
    record_type: crate::parser::RecordType,
) -> Result<(), crate::parser::ParseError> {
    if record_function & 0x00FF != (record_type as u16) & 0x00FF {
        return Err(crate::parser::ParseError::UnexpectedPattern {
            cause: format!(
                "The low-order byte of record_function \
                 `{record_function:#06X}` field must be same as `{:#06X}`",
                record_type as u16
            ),
        });
    }

    Ok(())
}

fn consume_remaining_bytes<R: crate::Read>(
    buf: &mut R,
    record_size: crate::parser::RecordSize,
) -> Result<(crate::imports::Vec<u8>, usize), crate::parser::ParseError> {
    if record_size.consumed_bytes() > record_size.byte_count() {
        return Err(crate::parser::ParseError::UnexpectedPattern {
            cause: alloc::format!(
                "consumed bytes ({}) exceeds record byte count ({})",
                record_size.consumed_bytes(),
                record_size.byte_count(),
            ),
        });
    }

    let remaining = record_size.remaining_bytes();
    if remaining == 0 {
        return Ok((crate::imports::Vec::new(), 0));
    }

    // Discard remaining bytes in fixed-size chunks to avoid
    // large allocations from crafted RecordSize values.
    let mut discarded = 0;
    let mut chunk = [0u8; 4096];

    while discarded < remaining {
        let to_read = core::cmp::min(remaining - discarded, chunk.len());

        match buf.read(&mut chunk[..to_read]) {
            Ok(n) if n == to_read => discarded += n,
            Ok(n) => {
                return Err(crate::parser::ParseError::FailedReadBuffer {
                    cause: crate::parser::ReadError::new(alloc::format!(
                        "expected {to_read} bytes read, but {n} bytes read"
                    )),
                });
            }
            Err(err) => {
                return Err(crate::parser::ParseError::FailedReadBuffer {
                    cause: crate::parser::ReadError::new(alloc::format!(
                        "{err:?}"
                    )),
                });
            }
        }
    }

    Ok((crate::imports::Vec::new(), discarded))
}

/// A 32-bit unsigned integer that defines the number of 16-bit WORD structures,
/// defined in [MS-DTYP] section 2.2.61, in the record.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct RecordSize(u32, usize);

/// Maximum allowed record size in WORDs.
/// 32 MW (64 MB) is far beyond any practical WMF record and guards
/// against crafted inputs that would cause excessive resource usage.
const MAX_RECORD_WORDS: u32 = 32 * 1024 * 1024;

impl RecordSize {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<Self, crate::parser::ParseError> {
        let (v, c) = crate::parser::read_u32_from_le_bytes(buf)?;

        if v > MAX_RECORD_WORDS {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "record size {v:#010X} exceeds maximum \
                     ({MAX_RECORD_WORDS:#010X})",
                ),
            });
        }

        Ok(Self(v, c))
    }
}

impl From<u32> for RecordSize {
    fn from(v: u32) -> Self {
        Self(v, 0)
    }
}

impl From<RecordSize> for u32 {
    fn from(v: RecordSize) -> Self {
        v.0
    }
}

impl core::fmt::Display for RecordSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#010X}", self.0)
    }
}

impl core::fmt::Debug for RecordSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "RecordSize(size: {:#010X}, consumed_bytes: {})",
            self.0, self.1
        )
    }
}

impl RecordSize {
    pub fn byte_count(&self) -> usize {
        (self.0 as usize).saturating_mul(2)
    }

    pub fn word_size(&self) -> usize {
        self.0 as usize
    }

    pub fn consume(&mut self, consumed_bytes: usize) {
        self.1 += consumed_bytes;
    }

    pub fn consumed_bytes(&self) -> usize {
        self.1
    }

    /// Returns true if consumed_bytes has exceeded byte_count,
    /// indicating a malformed record or a parser bug.
    pub fn is_overrun(&self) -> bool {
        self.1 > self.byte_count()
    }

    pub fn remaining(&self) -> bool {
        !self.is_overrun() && self.remaining_bytes() > 0
    }

    pub fn remaining_bytes(&self) -> usize {
        self.byte_count().saturating_sub(self.1)
    }
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
        let (record_function, c) =
            crate::parser::read_u16_from_le_bytes(&mut reader).unwrap();
        record_size.consume(c);
        (record_size, record_function, reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
