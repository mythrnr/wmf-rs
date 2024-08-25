#![allow(
    clippy::enum_variant_names,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    clippy::wildcard_imports,
    non_camel_case_types,
    non_snake_case
)]

mod constants;
mod objects;
mod records;

pub use self::{constants::*, objects::*, records::*};

#[derive(Clone, Debug, thiserror::Error)]
pub enum ParseError {
    #[error("failed to read buffer: {cause}")]
    FailedReadBuffer {
        #[source]
        cause: crate::ReadError,
    },
    #[error("not supported: {cause}")]
    NotSupported { cause: String },
    #[error("unexpected enum value: {cause}")]
    UnexpectedEnumValue { cause: String },
    #[error("unexpected bytes pattern: {cause}")]
    UnexpectedPattern { cause: String },
}

impl From<crate::ReadError> for ParseError {
    fn from(err: crate::ReadError) -> Self {
        Self::FailedReadBuffer { cause: err }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[error("failed to read buffer: {cause}")]
pub struct ReadError {
    cause: String,
}

impl ReadError {
    pub fn new(err: impl ToString) -> Self {
        Self { cause: err.to_string() }
    }
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> Self {
        Self::new(err)
    }
}

pub fn read<R: std::io::Read, const N: usize>(
    buf: &mut R,
) -> Result<([u8; N], usize), ReadError> {
    let mut buffer = [0u8; N];

    match buf.read(&mut buffer) {
        Ok(bytes_read) if bytes_read == N => Ok((buffer, N)),
        Ok(bytes_read) => Err(ReadError::new(format!(
            "expected {N} bytes read, but {bytes_read} bytes read"
        ))),
        Err(err) => Err(err.into()),
    }
}

pub fn read_variable<R: std::io::Read>(
    buf: &mut R,
    len: usize,
) -> Result<(Vec<u8>, usize), ReadError> {
    if len == 0 {
        return Ok((vec![0u8; 0], 0));
    }

    let mut buffer = vec![0u8; len];

    match buf.read(&mut buffer) {
        Ok(bytes_read) if bytes_read == len => Ok((buffer, len)),
        Ok(bytes_read) => Err(ReadError::new(format!(
            "expected {len} bytes read, but {bytes_read} bytes read"
        ))),
        Err(err) => Err(err.into()),
    }
}

macro_rules! impl_from_le_bytes {
    ($(($t:ty, $n:expr)),+) => {
        paste::paste!{
            $(
                pub fn [<read_ $t _from_le_bytes>]<R: ::std::io::Read>(
                    buf: &mut R,
                ) -> Result<($t, usize), ReadError> {
                    let (bytes, consumed_bytes) = read::<R, $n>(buf)?;

                    Ok((<$t>::from_le_bytes(bytes), consumed_bytes))
                }
            )*
        }
    };
}

impl_from_le_bytes! {(i8, 1), (i16, 2), (i32, 4), (u8, 1), (u16, 2), (u32, 4) }
