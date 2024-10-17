/// The Pen Object specifies the style, width, and color of a pen.
#[derive(Clone, Debug)]
pub struct Pen {
    /// PenStyle (2 bytes): A 16-bit unsigned integer that specifies the pen
    /// style. The value MUST be defined from the PenStyle Enumeration table.
    pub style: std::collections::BTreeSet<crate::parser::PenStyle>,
    /// Width (4 bytes): A 32-bit PointS Object that specifies a point for the
    /// object dimensions. The x-coordinate is the pen width. The y-coordinate
    /// is ignored.
    pub width: crate::parser::PointS,
    /// ColorRef (4 bytes): A 32-bit ColorRef Object that specifies the pen
    /// color value.
    pub color_ref: crate::parser::ColorRef,
}

impl Pen {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use strum::IntoEnumIterator;

        let (
            (style_u16, style_bytes),
            (width, width_bytes),
            (color_ref, color_ref_bytes),
        ) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::PointS::parse(buf)?,
            crate::parser::ColorRef::parse(buf)?,
        );

        let mut style = std::collections::BTreeSet::new();
        let (mut style_set, mut end_cap_set, mut join_set) =
            (false, false, false);

        for s in crate::parser::PenStyle::iter() {
            if style_u16 & s as u16 == s as u16 {
                if s as u16 <= crate::parser::PenStyle::PS_ALTERNATE as u16
                    && !style_set
                {
                    style.insert(s);
                    style_set = true;
                }

                if s as u16 <= crate::parser::PenStyle::PS_ENDCAP_FLAT as u16
                    && !end_cap_set
                {
                    style.insert(s);
                    end_cap_set = true;
                }

                if s as u16 <= crate::parser::PenStyle::PS_JOIN_MITER as u16
                    && !join_set
                {
                    style.insert(s);
                    join_set = true;
                }
            }
        }

        Ok((
            Self { style, width, color_ref },
            style_bytes + width_bytes + color_ref_bytes,
        ))
    }
}
