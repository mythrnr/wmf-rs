/// The Rect Object defines a rectangle.
#[derive(Clone, Debug)]
pub struct Rect {
    /// Left (2 bytes): A 16-bit signed integer that defines the x-coordinate,
    /// in logical coordinates, of the upper-left corner of the rectangle
    pub left: i16,
    /// Top (2 bytes): A 16-bit signed integer that defines the y-coordinate,
    /// in logical coordinates, of the upper-left corner of the rectangle.
    pub top: i16,
    /// Right (2 bytes): A 16-bit signed integer that defines the x-coordinate,
    /// in logical coordinates, of the lower-right corner of the rectangle.
    pub right: i16,
    /// Bottom (2 bytes): A 16-bit signed integer that defines the
    /// y-coordinate, in logical coordinates, of the lower-right corner of the
    /// rectangle.
    pub bottom: i16,
}

impl Rect {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (
            (left, left_bytes),
            (top, top_bytes),
            (right, right_bytes),
            (bottom, bottom_bytes),
        ) = (
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
        );

        Ok((
            Self { left, top, right, bottom },
            left_bytes + top_bytes + right_bytes + bottom_bytes,
        ))
    }
}
