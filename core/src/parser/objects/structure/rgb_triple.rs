/// The RGBTriple Object defines the pixel color values in an uncompressed DIB
/// Object.
#[derive(Clone, Debug)]
pub struct RGBTriple {
    /// Red (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of red.
    pub red: u8,
    /// Green (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of green.
    pub green: u8,
    /// Blue (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of blue.
    pub blue: u8,
}

impl RGBTriple {
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

        Ok((Self { red, green, blue }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let data = [0x11, 0x22, 0x33];
        let mut reader = &data[..];
        let (t, consumed) = RGBTriple::parse(&mut reader).unwrap();
        assert_eq!(t.red, 0x11);
        assert_eq!(t.green, 0x22);
        assert_eq!(t.blue, 0x33);
        assert_eq!(consumed, 3);
    }

    #[test]
    fn parse_truncated() {
        let data = [0x11];
        let mut reader = &data[..];
        assert!(RGBTriple::parse(&mut reader).is_err());
    }
}
