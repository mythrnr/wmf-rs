/// The Pen Object specifies the style, width, and color of a pen.
#[derive(Clone, Debug)]
pub struct Pen {
    /// PenStyle (2 bytes): A 16-bit unsigned integer that specifies the pen
    /// style. The value MUST be defined from the PenStyle Enumeration table.
    pub style: PenStyleSubsection,
    /// Width (4 bytes): A 32-bit PointS Object that specifies a point for the
    /// object dimensions. The x-coordinate is the pen width. The y-coordinate
    /// is ignored.
    pub width: crate::parser::PointS,
    /// ColorRef (4 bytes): A 32-bit ColorRef Object that specifies the pen
    /// color value.
    pub color_ref: crate::parser::ColorRef,
}

impl Pen {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_with;

        let mut consumed_bytes: usize = 0;
        let style =
            read_with(buf, &mut consumed_bytes, PenStyleSubsection::parse)?;
        let width =
            read_with(buf, &mut consumed_bytes, crate::parser::PointS::parse)?;
        let color_ref = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::ColorRef::parse,
        )?;

        Ok((Self { style, width, color_ref }, consumed_bytes))
    }
}

#[derive(Clone, Debug)]
pub struct PenStyleSubsection {
    pub end_cap: crate::parser::PenStyle,
    pub line_join: crate::parser::PenStyle,
    pub style: crate::parser::PenStyle,
    pub typ: crate::parser::PenStyle,
}

impl PenStyleSubsection {
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
        let style_u16: u16 = read_field(buf, &mut consumed_bytes)?;

        Ok((
            Self {
                // NOTE: use `PS_SOLID` as `PS_COSMETIC`.
                end_cap: Self::end_cap(style_u16),
                line_join: Self::line_join(style_u16),
                style: Self::style(style_u16),
                typ: crate::parser::PenStyle::PS_SOLID,
            },
            consumed_bytes,
        ))
    }

    fn end_cap(v: u16) -> crate::parser::PenStyle {
        const MASK: u16 = 0x0F00;

        for s in crate::parser::PenStyle::end_cap() {
            if v & MASK == s as u16 {
                return s;
            }
        }

        crate::parser::PenStyle::PS_ENDCAP_FLAT
    }

    pub fn line_join(v: u16) -> crate::parser::PenStyle {
        const MASK: u16 = 0xF000;

        for s in crate::parser::PenStyle::line_join() {
            if v & MASK == s as u16 {
                return s;
            }
        }

        crate::parser::PenStyle::PS_JOIN_MITER
    }

    pub fn style(v: u16) -> crate::parser::PenStyle {
        const MASK: u16 = 0x000F;

        for s in crate::parser::PenStyle::style() {
            if v & MASK == s as u16 {
                return s;
            }
        }

        crate::parser::PenStyle::PS_SOLID
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    /// Builds a 10-byte Pen payload (2 bytes style + 4 bytes PointS +
    /// 4 bytes ColorRef).
    fn build_pen(style_u16: u16, width_x: i16, color: [u8; 4]) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&style_u16.to_le_bytes());
        data.extend_from_slice(&width_x.to_le_bytes());
        data.extend_from_slice(&0_i16.to_le_bytes()); // y is ignored per spec
        data.extend_from_slice(&color);
        data
    }

    #[test]
    fn parse_solid_pen() {
        // style 0x0000 -> PS_SOLID across all sub-fields.
        let data = build_pen(0x0000, 5, [0xAA, 0xBB, 0xCC, 0x00]);
        let mut reader = &data[..];
        let (pen, consumed) = Pen::parse(&mut reader).unwrap();
        assert_eq!(pen.width.x, 5);
        assert_eq!(pen.color_ref.red, 0xAA);
        assert_eq!(pen.color_ref.green, 0xBB);
        assert_eq!(pen.color_ref.blue, 0xCC);
        assert!(matches!(pen.style.style, crate::parser::PenStyle::PS_SOLID));
        assert_eq!(consumed, 10);
    }

    #[test]
    fn parse_truncated() {
        let data = [0x00_u8, 0x00];
        let mut reader = &data[..];
        assert!(Pen::parse(&mut reader).is_err());
    }

    /// Verifies the bitmask routing for the dash style sub-field.
    /// PS_DASH = 0x0001 lives in the low nibble.
    #[test]
    fn pen_style_subsection_decodes_dash() {
        assert!(matches!(
            PenStyleSubsection::style(0x0001),
            crate::parser::PenStyle::PS_DASH,
        ));
    }
}
