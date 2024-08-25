/// The PointL Object defines the coordinates of a point.
#[derive(Clone, Debug)]
pub struct PointL {
    /// x (4 bytes): A 32-bit signed integer that defines the horizontal (x)
    /// coordinate of the point.
    pub x: i32,
    /// y (4 bytes): A 32-bit signed integer that defines the vertical (y)
    /// coordinate of the point.
    pub y: i32,
}

impl PointL {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let ((x, x_bytes), (y, y_bytes)) = (
            crate::read_i32_from_le_bytes(buf)?,
            crate::read_i32_from_le_bytes(buf)?,
        );

        Ok((Self { x, y }, x_bytes + y_bytes))
    }
}
