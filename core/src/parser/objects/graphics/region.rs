use crate::imports::*;

/// The Region Object defines a potentially non-rectilinear shape defined by an
/// array of scanlines.
#[derive(Clone, Debug)]
pub struct Region {
    /// nextInChain (2 bytes): A value that MUST be ignored. (Windows sets this
    /// field to `0x0000` .)
    pub next_in_chain: u16,
    /// ObjectType (2 bytes): A 16-bit signed integer that specifies the region
    /// identifier. It MUST be `0x0006`.
    pub object_type: i16,
    /// ObjectCount (4 bytes): A value that MUST be ignored. (Windows sets this
    /// field to an arbitrary value.)
    pub object_count: u32,
    /// RegionSize (2 bytes): A 16-bit signed integer that defines the size of
    /// the region in bytes plus the size of aScans in bytes.
    pub size: i16,
    /// ScanCount (2 bytes): A 16-bit signed integer that defines the number of
    /// scanlines composing the region.
    pub scan_count: i16,
    /// maxScan (2 bytes): A 16-bit signed integer that defines the maximum
    /// number of points in any one scan in this region.
    pub max_scan: i16,
    /// BoundingRectangle (8 bytes): A Rect Object that defines the bounding
    /// rectangle.
    pub bounding_rectangle: crate::parser::Rect,
    /// aScans (variable): An array of Scan Objects that define the scanlines
    /// in the region.
    pub a_scans: Vec<crate::parser::Scan>,
}

impl Region {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (next_in_chain, next_in_chain_bytes),
            (object_type, object_type_bytes),
            (object_count, object_count_bytes),
            (size, size_bytes),
            (scan_count, scan_count_bytes),
            (max_scan, max_scan_bytes),
            (bounding_rectangle, bounding_rectangle_bytes),
        ) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::Rect::parse(buf)?,
        );

        if object_type != 0x0006 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The object_type field must be 0x0006".to_owned(),
            });
        }

        if scan_count < 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "scan_count must be non-negative, got {scan_count}",
                ),
            });
        }

        let mut consumed_bytes = next_in_chain_bytes
            + object_type_bytes
            + object_count_bytes
            + size_bytes
            + scan_count_bytes
            + max_scan_bytes
            + bounding_rectangle_bytes;
        let mut a_scans = Vec::with_capacity(scan_count as usize);

        for _ in 0..scan_count {
            let (v, c) = crate::parser::Scan::parse(buf)?;

            consumed_bytes += c;
            a_scans.push(v);
        }

        Ok((
            Self {
                next_in_chain,
                object_type,
                object_count,
                size,
                scan_count,
                max_scan,
                bounding_rectangle,
                a_scans,
            },
            consumed_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_negative_scan_count() {
        let mut data = Vec::new();
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend_from_slice(&0x0006_i16.to_le_bytes());
        data.extend_from_slice(&0_u32.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&(-1_i16).to_le_bytes()); // negative scan_count
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&[0u8; 8]);
        let mut reader = &data[..];
        assert!(
            Region::parse(&mut reader).is_err(),
            "negative scan_count should be rejected"
        );
    }

    #[test]
    fn parse_bad_object_type() {
        let mut data = Vec::new();
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend_from_slice(&0x0005_i16.to_le_bytes()); // wrong
        data.extend_from_slice(&0_u32.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&[0u8; 8]);
        let mut reader = &data[..];
        assert!(
            Region::parse(&mut reader).is_err(),
            "object_type != 0x0006 should be rejected before reading scans"
        );
    }

    #[test]
    fn parse_zero_scans() {
        let mut data = Vec::new();
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend_from_slice(&0x0006_i16.to_le_bytes());
        data.extend_from_slice(&0_u32.to_le_bytes());
        data.extend_from_slice(&20_i16.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes()); // scan_count = 0
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes());
        data.extend_from_slice(&100_i16.to_le_bytes());
        data.extend_from_slice(&100_i16.to_le_bytes());
        let mut reader = &data[..];
        let (region, _) = Region::parse(&mut reader).unwrap();
        assert_eq!(region.scan_count, 0);
        assert!(region.a_scans.is_empty());
    }
}
