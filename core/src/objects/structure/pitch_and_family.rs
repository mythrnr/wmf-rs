/// The PitchAndFamily Object specifies the pitch and family properties of a
/// Font Object. Pitch refers to the width of the characters, and family refers
/// to the general appearance of a font.
#[derive(Clone, Debug)]
pub struct PitchAndFamily {
    /// Family (4 bits): A property of a font that describes its general
    /// appearance. This MUST be a value in the FamilyFont Enumeration.
    pub family: crate::FamilyFont,
    /// Pitch (2 bits): A property of a font that describes the pitch, of the
    /// characters. This MUST be a value in the PitchFont Enumeration.
    pub pitch: crate::PitchFont,
}

impl PitchAndFamily {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (byte, consumed_bytes) = crate::read_u8_from_le_bytes(buf)?;

        let family = byte >> 4;
        let Some(family) = crate::FamilyFont::from_repr(byte >> 4) else {
            return Err(crate::ParseError::UnexpectedEnumValue {
                cause: format!("unexpected value as FamilyFont: {family:#04X}"),
            });
        };

        let pitch = byte & 0b00000011;
        let Some(pitch) = crate::PitchFont::from_repr(pitch) else {
            return Err(crate::ParseError::UnexpectedEnumValue {
                cause: format!("unexpected value as PitchFont: {pitch:#04X}"),
            });
        };

        Ok((Self { family, pitch }, consumed_bytes))
    }
}
