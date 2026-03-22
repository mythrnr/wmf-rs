/// The PitchAndFamily Object specifies the pitch and family properties of a
/// Font Object. Pitch refers to the width of the characters, and family refers
/// to the general appearance of a font.
#[derive(Clone, Debug)]
pub struct PitchAndFamily {
    /// Family (4 bits): A property of a font that describes its general
    /// appearance. This MUST be a value in the FamilyFont Enumeration.
    pub family: crate::parser::FamilyFont,
    /// Pitch (2 bits): A property of a font that describes the pitch, of the
    /// characters. This MUST be a value in the PitchFont Enumeration.
    pub pitch: crate::parser::PitchFont,
}

impl PitchAndFamily {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (byte, consumed_bytes) = crate::parser::read_u8_from_le_bytes(buf)?;

        let family = byte >> 4;
        let Some(family) = crate::parser::FamilyFont::from_repr(byte >> 4)
        else {
            return Err(crate::parser::ParseError::UnexpectedEnumValue {
                cause: format!("unexpected value as FamilyFont: {family:#04X}"),
            });
        };

        let pitch = byte & 0b00000011;
        // The PitchFont field is 2 bits, but values beyond the
        // defined range (0-2) appear in real-world WMF files.
        // Fall back to DEFAULT_PITCH for undefined values.
        let pitch =
            crate::parser::PitchFont::from_repr(pitch).unwrap_or_else(|| {
                warn!(
                    pitch = format!("{pitch:#04X}"),
                    "undefined PitchFont value, falling back to DEFAULT_PITCH",
                );

                crate::parser::PitchFont::DEFAULT_PITCH
            });

        Ok((Self { family, pitch }, consumed_bytes))
    }
}
