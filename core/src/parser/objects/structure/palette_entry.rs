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
            v => Some(
                crate::parser::PaletteEntryFlag::from_repr(v).ok_or_else(
                    || crate::parser::ParseError::UnexpectedEnumValue {
                        cause: format!("invalid value {v} as PaletteEntryFlag")
                            .into(),
                    },
                )?,
            ),
        };

        Ok((Self { red, green, blue, values }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_zero_values() {
        let data = [0x10, 0x20, 0x30, 0x00];
        let mut reader = &data[..];
        let (entry, consumed) = PaletteEntry::parse(&mut reader).unwrap();
        assert_eq!(entry.red, 0x10);
        assert_eq!(entry.green, 0x20);
        assert_eq!(entry.blue, 0x30);
        assert!(entry.values.is_none());
        assert_eq!(consumed, 4);
    }

    #[test]
    fn parse_with_known_flag() {
        // 0x04 = PC_NOCOLLAPSE per PaletteEntryFlag enum.
        let data = [0x10, 0x20, 0x30, 0x04];
        let mut reader = &data[..];
        let (entry, _) = PaletteEntry::parse(&mut reader).unwrap();
        assert!(matches!(
            entry.values,
            Some(crate::parser::PaletteEntryFlag::PC_NOCOLLAPSE),
        ));
    }

    #[test]
    fn parse_rejects_invalid_flag() {
        // 0x99 is not in the PaletteEntryFlag enum.
        let data = [0x00, 0x00, 0x00, 0x99];
        let mut reader = &data[..];
        let err = PaletteEntry::parse(&mut reader).unwrap_err();
        assert!(matches!(
            err,
            crate::parser::ParseError::UnexpectedEnumValue { .. },
        ));
    }
}
