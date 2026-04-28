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
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let byte: u8 = read_field(buf, &mut consumed_bytes)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_family_default_pitch() {
        // family bits = 0x0 (FF_DONTCARE), pitch bits = 0 (DEFAULT_PITCH).
        let data = [0x00];
        let mut reader = &data[..];
        let (pf, consumed) = PitchAndFamily::parse(&mut reader).unwrap();
        assert!(matches!(pf.family, crate::parser::FamilyFont::FF_DONTCARE));
        assert!(matches!(pf.pitch, crate::parser::PitchFont::DEFAULT_PITCH));
        assert_eq!(consumed, 1);
    }

    #[test]
    fn parse_roman_fixed() {
        // family bits = 0x1 (FF_ROMAN), pitch bits = 0x1 (FIXED_PITCH).
        let data = [0x11];
        let mut reader = &data[..];
        let (pf, _) = PitchAndFamily::parse(&mut reader).unwrap();
        assert!(matches!(pf.family, crate::parser::FamilyFont::FF_ROMAN));
        assert!(matches!(pf.pitch, crate::parser::PitchFont::FIXED_PITCH));
    }

    #[test]
    fn parse_unknown_pitch_falls_back() {
        // family = 0x0 (FF_DONTCARE), pitch bits = 0x3 (out of range).
        // The parser must fall back to DEFAULT_PITCH per the comment in
        // the impl, instead of erroring.
        let data = [0x03];
        let mut reader = &data[..];
        let (pf, _) = PitchAndFamily::parse(&mut reader).unwrap();
        assert!(matches!(pf.pitch, crate::parser::PitchFont::DEFAULT_PITCH));
    }

    #[test]
    fn parse_rejects_unknown_family() {
        // Family bits = 0xF (no FamilyFont variant defined for that).
        let data = [0xF0];
        let mut reader = &data[..];
        let err = PitchAndFamily::parse(&mut reader).unwrap_err();
        assert!(matches!(
            err,
            crate::parser::ParseError::UnexpectedEnumValue { .. },
        ));
    }
}
