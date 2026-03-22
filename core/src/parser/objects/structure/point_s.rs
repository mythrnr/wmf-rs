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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((x, x_bytes), (y, y_bytes)) = (
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );

        Ok((Self { x, y }, x_bytes + y_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_positive() {
        let data = [100_i16.to_le_bytes(), 200_i16.to_le_bytes()].concat();
        let mut reader = &data[..];
        let (point, consumed) = PointS::parse(&mut reader).unwrap();
        assert_eq!(point.x, 100);
        assert_eq!(point.y, 200);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn parse_negative() {
        let data = [(-50_i16).to_le_bytes(), (-100_i16).to_le_bytes()].concat();
        let mut reader = &data[..];
        let (point, _) = PointS::parse(&mut reader).unwrap();
        assert_eq!(point.x, -50);
        assert_eq!(point.y, -100);
    }

    #[test]
    fn parse_zero() {
        let data = [0_i16.to_le_bytes(), 0_i16.to_le_bytes()].concat();
        let mut reader = &data[..];
        let (point, _) = PointS::parse(&mut reader).unwrap();
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);
    }

    #[test]
    fn parse_insufficient_buffer() {
        let data = [100_i16.to_le_bytes()].concat();
        let mut reader = &data[..];
        assert!(PointS::parse(&mut reader).is_err());
    }
}
