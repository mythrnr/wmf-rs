//! Implementation of the definitions in Section 2.3.2 of the WMF
//! specifications.

mod eof;
mod header;
mod placeable;

pub use self::{eof::*, header::*, placeable::*};

#[derive(Clone, Debug)]
pub enum MetafileHeader {
    StartsWithPlaceable(
        crate::parser::META_PLACEABLE,
        crate::parser::META_HEADER,
    ),
    StartsWithHeader(crate::parser::META_HEADER),
}

impl MetafileHeader {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let mut key: u32 = read_field(buf, &mut consumed_bytes)?;

        let placeable = if key == 0x9AC6CDD7 {
            // Reset the byte counter to match the original behavior:
            // when a placeable header is detected, only the post-placeable
            // read (placeable + new key) is reported, dropping the initial
            // key read.
            consumed_bytes = 0;
            let v = read_with(buf, &mut consumed_bytes, |b| {
                crate::parser::META_PLACEABLE::parse(b, key)
            })?;
            key = read_field(buf, &mut consumed_bytes)?;

            Some(v)
        } else {
            None
        };

        let header = read_with(buf, &mut consumed_bytes, |b| {
            crate::parser::META_HEADER::parse(b, key)
        })?;

        Ok(if let Some(placeable) = placeable {
            (Self::StartsWithPlaceable(placeable, header), consumed_bytes)
        } else {
            (Self::StartsWithHeader(header), consumed_bytes)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    #[test]
    fn parse_with_placeable() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x9AC6CDD7_u32.to_le_bytes());
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&1024_i16.to_le_bytes());
        data.extend_from_slice(&768_i16.to_le_bytes());
        data.extend_from_slice(&1440_u16.to_le_bytes());
        data.extend_from_slice(&0_u32.to_le_bytes());
        data.extend_from_slice(&[0u8; 2]);
        // META_HEADER
        data.extend_from_slice(&0x0001_u16.to_le_bytes());
        data.extend_from_slice(&9_u16.to_le_bytes());
        data.extend_from_slice(&0x0300_u16.to_le_bytes());
        data.extend_from_slice(&100_u16.to_le_bytes());
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend_from_slice(&5_u16.to_le_bytes());
        data.extend_from_slice(&50_u32.to_le_bytes());
        data.extend_from_slice(&0_u16.to_le_bytes());

        let mut reader = &data[..];
        let (header, _) = MetafileHeader::parse(&mut reader).unwrap();
        match header {
            MetafileHeader::StartsWithPlaceable(placeable, header) => {
                assert_eq!(placeable.key, 0x9AC6CDD7);
                assert_eq!(placeable.bounding_box.right, 1024);
                assert_eq!(header.number_of_objects, 5);
            }
            MetafileHeader::StartsWithHeader(_) => {
                panic!("expected StartsWithPlaceable");
            }
        }
    }

    #[test]
    fn parse_without_placeable() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x0001_u16.to_le_bytes());
        data.extend_from_slice(&9_u16.to_le_bytes());
        data.extend_from_slice(&0x0300_u16.to_le_bytes());
        data.extend_from_slice(&50_u16.to_le_bytes());
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend_from_slice(&3_u16.to_le_bytes());
        data.extend_from_slice(&20_u32.to_le_bytes());
        data.extend_from_slice(&0_u16.to_le_bytes());

        let mut reader = &data[..];
        let (header, _) = MetafileHeader::parse(&mut reader).unwrap();
        match header {
            MetafileHeader::StartsWithHeader(header) => {
                assert_eq!(header.number_of_objects, 3);
            }
            MetafileHeader::StartsWithPlaceable(..) => {
                panic!("expected StartsWithHeader");
            }
        }
    }

    #[test]
    fn parse_invalid_number_of_members() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x0001_u16.to_le_bytes());
        data.extend_from_slice(&9_u16.to_le_bytes());
        data.extend_from_slice(&0x0300_u16.to_le_bytes());
        data.extend_from_slice(&50_u16.to_le_bytes());
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend_from_slice(&3_u16.to_le_bytes());
        data.extend_from_slice(&20_u32.to_le_bytes());
        data.extend_from_slice(&1_u16.to_le_bytes()); // invalid

        let mut reader = &data[..];
        assert!(MetafileHeader::parse(&mut reader).is_err());
    }

    #[test]
    fn parse_empty_buffer() {
        let mut reader: &[u8] = &[];
        assert!(MetafileHeader::parse(&mut reader).is_err());
    }
}
