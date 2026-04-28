use crate::imports::*;

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

        if reserved != 0x00 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The reserved field must be zero".to_owned(),
            });
        }

        Ok((Self { red, green, blue, reserved }, consumed_bytes))
    }
}
