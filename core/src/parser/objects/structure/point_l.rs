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
        let x = read_field(buf, &mut consumed_bytes)?;
        let y = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { x, y }, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    #[test]
    fn parse_ok() {
        let mut data = Vec::new();
        data.extend_from_slice(&(-12345_i32).to_le_bytes());
        data.extend_from_slice(&67890_i32.to_le_bytes());
        let mut reader = &data[..];
        let (point, consumed) = PointL::parse(&mut reader).unwrap();
        assert_eq!(point.x, -12345);
        assert_eq!(point.y, 67890);
        assert_eq!(consumed, 8);
    }

    #[test]
    fn parse_truncated() {
        // Only 4 bytes -> y read fails partway through.
        let data = 1_i32.to_le_bytes();
        let mut reader = &data[..];
        assert!(PointL::parse(&mut reader).is_err());
    }
}
