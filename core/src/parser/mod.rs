mod constants;
mod objects;
mod records;

pub use self::{constants::*, objects::*, records::*};
use crate::imports::*;

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum ParseError {
    #[snafu(display("failed to read buffer: {cause}"))]
    FailedReadBuffer { cause: ReadError },
    #[snafu(display("not supported: {cause}"))]
    NotSupported { cause: String },
    #[snafu(display("unexpected enum value: {cause}"))]
    UnexpectedEnumValue { cause: String },
    #[snafu(display("unexpected bytes pattern: {cause}"))]
    UnexpectedPattern { cause: String },
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
    if len == 0 {
        return Ok((Vec::new(), 0));
    }

    let mut buffer = vec![0u8; len];

    read_exact(buf, &mut buffer)?;

    Ok((buffer, len))
}

/// Reads exactly `buffer.len()` bytes from `buf`, looping to handle
/// short reads that `embedded_io::Read` may return.
fn read_exact<R: crate::Read>(
    buf: &mut R,
    buffer: &mut [u8],
) -> Result<(), ReadError> {
    let total = buffer.len();
    let mut offset = 0;

    while offset < total {
        match buf.read(&mut buffer[offset..]) {
            Ok(0) => {
                return Err(ReadError::new(format!(
                    "expected {total} bytes read, but {offset} bytes read \
                     (unexpected end of stream)"
                )));
            }
            Ok(n) => offset += n,
            Err(err) => return Err(ReadError::new(format!("{err:?}"))),
        }
    }

    Ok(())
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
    fn track(&mut self, consumed_bytes: usize) {
        *self += consumed_bytes;
    }
}

impl ConsumeTracker for crate::parser::RecordSize {
    fn track(&mut self, consumed_bytes: usize) {
        self.consume(consumed_bytes);
    }
}

impl ReadLeField for i8 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((i8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i16 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((i16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for i32 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 4>(buf)?;
        Ok((i32::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u8 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 1>(buf)?;
        Ok((u8::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u16 {
    fn read_le<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), ReadError> {
        let (bytes, c) = read::<R, 2>(buf)?;
        Ok((u16::from_le_bytes(bytes), c))
    }
}

impl ReadLeField for u32 {
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
            .filter_map(|v| {
                crate::parser::symbol_charset_table().get(v).copied()
            })
            .collect::<String>())
    } else {
        let encoding: &'static encoding_rs::Encoding = charset.into();
        let (cow, _, had_errors) = encoding.decode(bytes);

        if had_errors {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "Failed to decode string with specified charset"
                    .to_string(),
            });
        }

        Ok(cow.replace('\0', ""))
    }
}

#[cfg(test)]
mod tests {
    use super::ReadLeField;

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
