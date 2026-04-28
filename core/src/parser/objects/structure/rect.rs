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

    pub fn overlap(&self, other: &Rect) -> Option<Rect> {
        let left = self.left.max(other.left);
        let top = self.top.min(other.top);
        let right = self.right.min(other.right);
        let bottom = self.bottom.max(other.bottom);

        if left < right && bottom < top {
            Some(Rect { left, top, right, bottom })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let data = [
            10_i16.to_le_bytes(),
            20_i16.to_le_bytes(),
            100_i16.to_le_bytes(),
            200_i16.to_le_bytes(),
        ]
        .concat();
        let mut reader = &data[..];
        let (rect, consumed) = Rect::parse(&mut reader).unwrap();
        assert_eq!(rect.left, 10);
        assert_eq!(rect.top, 20);
        assert_eq!(rect.right, 100);
        assert_eq!(rect.bottom, 200);
        assert_eq!(consumed, 8);
    }

    #[test]
    fn parse_insufficient_buffer() {
        let data = [10_i16.to_le_bytes(), 20_i16.to_le_bytes()].concat();
        let mut reader = &data[..];
        assert!(Rect::parse(&mut reader).is_err());
    }
}
