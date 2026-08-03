/// The CIEXYZ Object defines information about the CIEXYZ chromaticity object.
#[derive(Clone, Debug)]
pub struct CIEXYZ {
    /// ciexyzX (4 bytes): A 32-bit 2.30 fixed point type that defines the x
    /// chromaticity value.
    pub x: u32,
    /// ciexyzY (4 bytes): A 32-bit 2.30 fixed point type that defines the y
    /// chromaticity value.
    pub y: u32,
    /// ciexyzZ (4 bytes): A 32-bit 2.30 fixed point type that defines the z
    /// chromaticity value.
    pub z: u32,
}

impl CIEXYZ {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let x = read_field(buf, &mut consumed_bytes)?;
        let y = read_field(buf, &mut consumed_bytes)?;
        let z = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { x, y, z }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    #[test]
    fn parse_ok() {
        let mut data = Vec::new();
        for v in [1_u32, 2, 3] {
            data.extend_from_slice(&v.to_le_bytes());
        }
        let mut reader = &data[..];
        let (c, consumed) = CIEXYZ::parse(&mut reader).unwrap();
        assert_eq!(c.x, 1);
        assert_eq!(c.y, 2);
        assert_eq!(c.z, 3);
        assert_eq!(consumed, 12);
    }

    #[test]
    fn parse_truncated() {
        let data = 1_u32.to_le_bytes();
        let mut reader = &data[..];
        assert!(CIEXYZ::parse(&mut reader).is_err());
    }
}
