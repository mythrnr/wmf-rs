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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (red, red_bytes),
            (green, green_bytes),
            (blue, blue_bytes),
            (reserved, reserved_bytes),
        ) = (
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
        );

        if reserved != 0x00 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The reserved field must be zero".to_owned(),
            });
        }

        Ok((
            Self { red, green, blue, reserved },
            red_bytes + green_bytes + blue_bytes + reserved_bytes,
        ))
    }
}
