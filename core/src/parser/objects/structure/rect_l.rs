/// The RectL Object defines a rectangle.
///
/// A rectangle defined with a RectL Object is filled up to— but not
/// including—the right column and bottom row of pixels.
#[derive(Clone, Debug)]
pub struct RectL {
    /// Left (4 bytes): A 32-bit signed integer that defines the x coordinate,
    /// in logical coordinates, of the upper-left corner of the rectangle.
    pub left: i32,
    /// Top (4 bytes): A 32-bit signed integer that defines the y coordinate,
    /// in logical coordinates, of the upper-left corner of the rectangle.
    pub top: i32,
    /// Right (4 bytes): A 32-bit signed integer that defines the x coordinate,
    /// in logical coordinates, of the lower-right corner of the rectangle.
    pub right: i32,
    /// Bottom (4 bytes): A 32-bit signed integer that defines y coordinate, in
    /// logical coordinates, of the lower-right corner of the rectangle.
    pub bottom: i32,
}

impl RectL {
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
        let left = read_field(buf, &mut consumed_bytes)?;
        let top = read_field(buf, &mut consumed_bytes)?;
        let right = read_field(buf, &mut consumed_bytes)?;
        let bottom = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { left, top, right, bottom }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    #[test]
    fn parse_ok() {
        let mut data = Vec::new();
        for v in [10_i32, 20, 110, 220] {
            data.extend_from_slice(&v.to_le_bytes());
        }
        let mut reader = &data[..];
        let (rect, consumed) = RectL::parse(&mut reader).unwrap();
        assert_eq!(rect.left, 10);
        assert_eq!(rect.top, 20);
        assert_eq!(rect.right, 110);
        assert_eq!(rect.bottom, 220);
        assert_eq!(consumed, 16);
    }

    #[test]
    fn parse_truncated() {
        let data = 10_i32.to_le_bytes();
        let mut reader = &data[..];
        assert!(RectL::parse(&mut reader).is_err());
    }
}
