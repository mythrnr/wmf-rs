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
