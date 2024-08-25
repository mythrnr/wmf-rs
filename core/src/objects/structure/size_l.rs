/// The SizeL Object defines the x- and y-extents of a rectangle.
#[derive(Clone, Debug)]
pub struct SizeL {
    /// cx (4 bytes): A 32-bit unsigned integer that defines the x-coordinate
    /// of the point.
    pub cx: u32,
    /// cy (4 bytes): A 32-bit unsigned integer that defines the y-coordinate
    /// of the point.
    pub cy: u32,
}

impl SizeL {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let ((cx, cx_bytes), (cy, cy_bytes)) = (
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
        );

        Ok((Self { cx, cy }, cx_bytes + cy_bytes))
    }
}
