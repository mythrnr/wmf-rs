use crate::imports::*;

/// The Palette Object specifies the colors in a logical palette.
#[derive(Clone, Debug)]
pub struct Palette {
    /// Start (2 bytes): A 16-bit unsigned integer that defines the offset into
    /// the Palette Object when used with the META_SETPALENTRIES and
    /// META_ANIMATEPALETTE record types. When used with META_CREATEPALETTE
    /// record type, it MUST be 0x0300.
    pub start: u16,
    /// NumberOfEntries (2 bytes): A 16-bit unsigned integer that defines the
    /// number of objects in aPaletteEntries.
    pub number_of_entries: u16,
    /// aPaletteEntries (variable): An array of NumberOfEntries 32-bit
    /// PaletteEntry Objects.
    pub a_palette_entries: Vec<crate::parser::PaletteEntry>,
}

impl Palette {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let start = read_field(buf, &mut consumed_bytes)?;
        let number_of_entries = read_field(buf, &mut consumed_bytes)?;

        let mut a_palette_entries =
            Vec::with_capacity(number_of_entries as usize);

        for _ in 0..number_of_entries {
            let v = read_with(
                buf,
                &mut consumed_bytes,
                crate::parser::PaletteEntry::parse,
            )?;
            a_palette_entries.push(v);
        }

        Ok((
            Self { start, number_of_entries, a_palette_entries },
            consumed_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_two_entries() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x0300_u16.to_le_bytes()); // start
        data.extend_from_slice(&2_u16.to_le_bytes()); // number_of_entries
        // Entry 1
        data.extend_from_slice(&[0x10, 0x20, 0x30, 0x00]);
        // Entry 2
        data.extend_from_slice(&[0x40, 0x50, 0x60, 0x00]);
        let mut reader = &data[..];
        let (p, consumed) = Palette::parse(&mut reader).unwrap();
        assert_eq!(p.start, 0x0300);
        assert_eq!(p.number_of_entries, 2);
        assert_eq!(p.a_palette_entries.len(), 2);
        assert_eq!(p.a_palette_entries[0].red, 0x10);
        assert_eq!(p.a_palette_entries[1].green, 0x50);
        assert_eq!(consumed, 4 + 4 + 4);
    }

    #[test]
    fn parse_with_zero_entries() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x0300_u16.to_le_bytes());
        data.extend_from_slice(&0_u16.to_le_bytes());
        let mut reader = &data[..];
        let (p, _) = Palette::parse(&mut reader).unwrap();
        assert!(p.a_palette_entries.is_empty());
    }

    #[test]
    fn parse_truncated_entries() {
        // Header claims 5 entries but only 1 follows.
        let mut data = Vec::new();
        data.extend_from_slice(&0x0300_u16.to_le_bytes());
        data.extend_from_slice(&5_u16.to_le_bytes());
        data.extend_from_slice(&[0x00; 4]);
        let mut reader = &data[..];
        assert!(Palette::parse(&mut reader).is_err());
    }
}
