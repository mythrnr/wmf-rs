/// The PaletteEntry Object defines the color and usage of an entry in a
/// palette.
#[derive(Clone, Debug)]
pub struct PaletteEntry {
    /// Red (1 byte): An 8-bit unsigned integer that defines the red intensity
    /// value for the palette entry.
    pub red: u8,
    /// Green (1 byte): An 8-bit unsigned integer that defines the green
    /// intensity value for the palette entry.
    pub green: u8,
    /// Blue (1 byte): An 8-bit unsigned integer that defines the blue
    /// intensity value for the palette entry.
    pub blue: u8,
    /// Values (1 byte): An 8-bit unsigned integer that defines how the palette
    /// entry is to be used. The Values field MUST be 0x00 or one of the values
    /// in the PaletteEntryFlag Enumeration table.
    pub values: Option<crate::parser::PaletteEntryFlag>,
}

impl PaletteEntry {
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
        let values = read_field(buf, &mut consumed_bytes)?;

        let values = match values {
            0x00 => None,
            v => {
                Some(crate::parser::PaletteEntryFlag::from_repr(v).ok_or_else(
                    || crate::parser::ParseError::UnexpectedEnumValue {
                        cause: format!("invalid value {v} as PaletteEntryFlag"),
                    },
                )?)
            }
        };

        Ok((Self { red, green, blue, values }, consumed_bytes))
    }
}
