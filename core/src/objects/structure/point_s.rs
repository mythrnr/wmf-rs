/// The PointS Object defines the x- and y-coordinates of a point.
#[derive(Clone, Debug)]
pub struct PointS {
    /// x (2 bytes): A 16-bit signed integer that defines the horizontal (x)
    /// coordinate of the point.
    pub x: i16,
    /// y (2 bytes): A 16-bit signed integer that defines the vertical (y)
    /// coordinate of the point.
    pub y: i16,
}

impl PointS {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let ((x, x_bytes), (y, y_bytes)) = (
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
        );

        Ok((Self { x, y }, x_bytes + y_bytes))
    }
}
