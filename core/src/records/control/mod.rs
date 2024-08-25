//! Implementation of the definitions in Section 2.3.2 of the WMF
//! specifications.

mod eof;
mod header;
mod replaceable;

pub use self::{eof::*, header::*, replaceable::*};

#[derive(Clone, Debug)]
pub enum MetafileHeader {
    StartsWithPlaceable(crate::META_PLACEABLE, crate::META_HEADER),
    StartsWithHeader(crate::META_HEADER),
}

impl MetafileHeader {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (mut key, mut consumed_bytes) = crate::read_u32_from_le_bytes(buf)?;

        let placeable = if key == 0x9AC6CDD7 {
            let (v, c) = crate::META_PLACEABLE::parse(buf, key)?;
            let (k, key_bytes) = crate::read_u32_from_le_bytes(buf)?;

            (key, consumed_bytes) = (k, c + key_bytes);

            Some(v)
        } else {
            None
        };

        let (header, c) = crate::META_HEADER::parse(buf, key)?;
        consumed_bytes += c;

        Ok(if let Some(placeable) = placeable {
            (Self::StartsWithPlaceable(placeable, header), consumed_bytes)
        } else {
            (Self::StartsWithHeader(header), consumed_bytes)
        })
    }
}
