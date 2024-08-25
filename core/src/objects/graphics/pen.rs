/// The Pen Object specifies the style, width, and color of a pen.
#[derive(Clone, Debug)]
pub struct Pen {
    /// PenStyle (2 bytes): A 16-bit unsigned integer that specifies the pen
    /// style. The value MUST be defined from the PenStyle Enumeration table.
    pub style: crate::PenStyle,
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
        let (
            (style, style_bytes),
            (width, width_bytes),
            (color_ref, color_ref_bytes),
        ) = (
            crate::PenStyle::parse(buf)?,
            crate::PointS::parse(buf)?,
            crate::ColorRef::parse(buf)?,
        );

        Ok((
            Self { style, width, color_ref },
            style_bytes + width_bytes + color_ref_bytes,
        ))
    }
}
