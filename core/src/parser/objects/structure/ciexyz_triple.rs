/// The CIEXYZTriple Object defines information about the CIEXYZTriple color
/// object.
#[derive(Clone, Debug)]
pub struct CIEXYZTriple {
    /// ciexyzRed (12 bytes): A 96-bit CIEXYZ Object (section 2.2.2.6) that
    /// defines the red chromaticity values.
    pub red: crate::parser::CIEXYZ,
    /// ciexyzGreen (12 bytes): A 96-bit CIEXYZ Object that defines the green
    /// chromaticity values.
    pub green: crate::parser::CIEXYZ,
    /// ciexyzBlue (12 bytes): A 96-bit CIEXYZ Object that defines the blue
    /// chromaticity values.
    pub blue: crate::parser::CIEXYZ,
}

impl CIEXYZTriple {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((red, red_bytes), (green, green_bytes), (blue, blue_bytes)) = (
            crate::parser::CIEXYZ::parse(buf)?,
            crate::parser::CIEXYZ::parse(buf)?,
            crate::parser::CIEXYZ::parse(buf)?,
        );
        let consumed_bytes = red_bytes + green_bytes + blue_bytes;

        Ok((Self { red, green, blue }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    #[test]
    fn parse_ok() {
        let mut data = Vec::new();
        // 9 u32 values for red/green/blue CIEXYZ
        for v in 1_u32..=9 {
            data.extend_from_slice(&v.to_le_bytes());
        }
        let mut reader = &data[..];
        let (t, consumed) = CIEXYZTriple::parse(&mut reader).unwrap();
        assert_eq!(t.red.x, 1);
        assert_eq!(t.red.y, 2);
        assert_eq!(t.red.z, 3);
        assert_eq!(t.green.x, 4);
        assert_eq!(t.blue.z, 9);
        assert_eq!(consumed, 36);
    }

    #[test]
    fn parse_truncated() {
        // Only 8 bytes -> red CIEXYZ parse fails on z field.
        let data = [0u8; 8];
        let mut reader = &data[..];
        assert!(CIEXYZTriple::parse(&mut reader).is_err());
    }
}
