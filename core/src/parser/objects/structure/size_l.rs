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
        let cx = read_field(buf, &mut consumed_bytes)?;
        let cy = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { cx, cy }, consumed_bytes))
    }
}
