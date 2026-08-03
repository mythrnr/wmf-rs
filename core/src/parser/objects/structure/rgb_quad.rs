/// The RGBQuad Object defines the pixel color values in an uncompressed DIB
/// Object.
#[derive(Clone, Debug)]
pub struct RGBQuad {
    /// Red (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of red.
    pub red: u8,
    /// Green (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of green.
    pub green: u8,
    /// Blue (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of blue.
    pub blue: u8,
    /// Reserved (1 byte): An 8-bit unsigned integer that MUST be 0x00.
    pub reserved: u8,
}

impl RGBQuad {
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
        let red = read_field(buf, &mut consumed_bytes)?;
        let green = read_field(buf, &mut consumed_bytes)?;
        let blue = read_field(buf, &mut consumed_bytes)?;
        let reserved = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq("reserved", reserved, 0x00_u8)?;

        Ok((Self { red, green, blue, reserved }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let data = [0x10, 0x20, 0x30, 0x00];
        let mut reader = &data[..];
        let (q, consumed) = RGBQuad::parse(&mut reader).unwrap();
        assert_eq!(q.red, 0x10);
        assert_eq!(q.green, 0x20);
        assert_eq!(q.blue, 0x30);
        assert_eq!(q.reserved, 0x00);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn parse_rejects_nonzero_reserved() {
        let data = [0x00, 0x00, 0x00, 0xFF];
        let mut reader = &data[..];
        let err = RGBQuad::parse(&mut reader).unwrap_err();
        assert!(matches!(err, crate::parser::ParseError::MismatchedField {
            field: "reserved",
            ..
        },));
    }

    #[test]
    fn parse_truncated() {
        let data = [0x00, 0x00];
        let mut reader = &data[..];
        assert!(RGBQuad::parse(&mut reader).is_err());
    }
}
