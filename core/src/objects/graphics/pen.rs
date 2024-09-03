/// The Pen Object specifies the style, width, and color of a pen.
#[derive(Clone, Debug)]
pub struct Pen {
    /// PenStyle (2 bytes): A 16-bit unsigned integer that specifies the pen
    /// style. The value MUST be defined from the PenStyle Enumeration table.
    pub style: std::collections::BTreeSet<crate::PenStyle>,
    /// Width (4 bytes): A 32-bit PointS Object that specifies a point for the
    /// object dimensions. The x-coordinate is the pen width. The y-coordinate
    /// is ignored.
    pub width: crate::PointS,
    /// ColorRef (4 bytes): A 32-bit ColorRef Object that specifies the pen
    /// color value.
    pub color_ref: crate::ColorRef,
}

impl Pen {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        use strum::IntoEnumIterator;

        let (
            (style_u16, style_bytes),
            (width, width_bytes),
            (color_ref, color_ref_bytes),
        ) = (
            crate::read_u16_from_le_bytes(buf)?,
            crate::PointS::parse(buf)?,
            crate::ColorRef::parse(buf)?,
        );

        let mut style = std::collections::BTreeSet::new();
        let mut iter = crate::PenStyle::iter();
        let (mut style_set, mut end_cap_set, mut join_set) =
            (false, false, false);

        while let Some(s) = iter.next() {
            if style_u16 & s as u16 == s as u16 {
                if s as u16 <= crate::PenStyle::PS_ALTERNATE as u16 {
                    if !style_set {
                        style.insert(s);
                        style_set = true;
                    }
                }

                if s as u16 <= crate::PenStyle::PS_ENDCAP_FLAT as u16 {
                    if !end_cap_set {
                        style.insert(s);
                        end_cap_set = true;
                    }
                }

                if s as u16 <= crate::PenStyle::PS_JOIN_MITER as u16 {
                    if !join_set {
                        style.insert(s);
                        join_set = true;
                    }
                }
            }
        }

        Ok((
            Self { style, width, color_ref },
            style_bytes + width_bytes + color_ref_bytes,
        ))
    }
}
