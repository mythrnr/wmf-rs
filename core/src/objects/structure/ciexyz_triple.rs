/// The CIEXYZTriple Object defines information about the CIEXYZTriple color
/// object.
#[derive(Clone, Debug)]
pub struct CIEXYZTriple {
    /// ciexyzRed (12 bytes): A 96-bit CIEXYZ Object (section 2.2.2.6) that
    /// defines the red chromaticity values.
    pub red: crate::CIEXYZ,
    /// ciexyzGreen (12 bytes): A 96-bit CIEXYZ Object that defines the green
    /// chromaticity values.
    pub green: crate::CIEXYZ,
    /// ciexyzBlue (12 bytes): A 96-bit CIEXYZ Object that defines the blue
    /// chromaticity values.
    pub blue: crate::CIEXYZ,
}

impl CIEXYZTriple {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let ((red, red_bytes), (green, green_bytes), (blue, blue_bytes)) = (
            crate::CIEXYZ::parse(buf)?,
            crate::CIEXYZ::parse(buf)?,
            crate::CIEXYZ::parse(buf)?,
        );
        let consumed_bytes = red_bytes + green_bytes + blue_bytes;

        Ok((Self { red, green, blue }, consumed_bytes))
    }
}
