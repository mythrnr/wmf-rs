use crate::imports::*;

/// The PolyPolygon Object defines a series of closed polygons.
#[derive(Clone, Debug)]
pub struct PolyPolygon {
    /// NumberOfPolygons (2 bytes): A 16-bit unsigned integer that defines the
    /// number of polygons in the object.
    pub number_of_polygons: u16,
    /// aPointsPerPolygon (variable): A NumberOfPolygons array of 16-bit
    /// unsigned integers that define the number of points for each polygon in
    /// the object.
    pub a_points_per_polygon: Vec<u16>,
    /// aPoints (variable): An array of PointS values that define the
    /// coordinates of the polygons. The length of the array is equal to the
    /// sum of all 16-bit integers in the aPointsPerPolygon array.
    pub a_points: Vec<crate::parser::PointS>,
}

impl PolyPolygon {
    /// Upper bound for the total number of points a single PolyPolygon may
    /// contain. Derived from the parser-wide record size limit (64 MiB)
    /// divided by the size of `PointS` (4 bytes), so that a crafted input
    /// cannot trigger an oversized `Vec::with_capacity` allocation that
    /// ignores the actual payload size.
    const MAX_TOTAL_POINTS: u32 = 16 * 1024 * 1024;

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let number_of_polygons = read_field(buf, &mut consumed_bytes)?;
        let mut number_of_points: u32 = 0;
        let mut a_points_per_polygon =
            Vec::with_capacity(number_of_polygons as usize);

        for _ in 0..number_of_polygons {
            let v = read_field(buf, &mut consumed_bytes)?;

            // Use checked_add to keep the safety invariant explicit even
            // though u16::MAX * u16::MAX fits in u32; the bound check below
            // is the real guard against memory exhaustion.
            number_of_points = number_of_points
                .checked_add(u32::from(v))
                .ok_or(crate::parser::ParseError::UnexpectedPattern {
                    cause: "sum of points per polygon overflowed u32"
                        .to_string(),
                })?;
            a_points_per_polygon.push(v);
        }

        if number_of_points > Self::MAX_TOTAL_POINTS {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "total point count {number_of_points} exceeds maximum {}",
                    Self::MAX_TOTAL_POINTS,
                ),
            });
        }

        let mut a_points = Vec::with_capacity(number_of_points as usize);

        for _ in 0..number_of_points {
            let v = read_with(
                buf,
                &mut consumed_bytes,
                crate::parser::PointS::parse,
            )?;
            a_points.push(v);
        }

        Ok((
            Self { number_of_polygons, a_points_per_polygon, a_points },
            consumed_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_two_triangles() {
        let mut data = Vec::new();
        data.extend_from_slice(&2_u16.to_le_bytes());
        data.extend_from_slice(&3_u16.to_le_bytes());
        data.extend_from_slice(&3_u16.to_le_bytes());
        for i in 0..6_i16 {
            data.extend_from_slice(&i.to_le_bytes());
            data.extend_from_slice(&(i * 10).to_le_bytes());
        }
        let mut reader = &data[..];
        let (poly, _) = PolyPolygon::parse(&mut reader).unwrap();
        assert_eq!(poly.number_of_polygons, 2);
        assert_eq!(poly.a_points_per_polygon, vec![3, 3]);
        assert_eq!(poly.a_points.len(), 6);
    }

    #[test]
    fn parse_large_point_count_no_overflow() {
        let mut data = Vec::new();
        data.extend_from_slice(&3_u16.to_le_bytes());
        data.extend_from_slice(&30000_u16.to_le_bytes());
        data.extend_from_slice(&30000_u16.to_le_bytes());
        data.extend_from_slice(&30000_u16.to_le_bytes());
        let mut reader = &data[..];
        // Should fail with read error (not panic from overflow).
        assert!(PolyPolygon::parse(&mut reader).is_err());
    }

    #[test]
    fn parse_total_point_count_exceeds_max_is_rejected_before_alloc() {
        // Construct a polygon set whose total point count exceeds
        // PolyPolygon::MAX_TOTAL_POINTS, and verify the parser rejects it
        // on the bound check rather than attempting the `a_points` alloc.
        let polygon_count: u16 =
            (PolyPolygon::MAX_TOTAL_POINTS / u32::from(u16::MAX) + 1) as u16;
        let mut data = Vec::new();
        data.extend_from_slice(&polygon_count.to_le_bytes());
        for _ in 0..polygon_count {
            data.extend_from_slice(&u16::MAX.to_le_bytes());
        }
        let mut reader = &data[..];
        let err = PolyPolygon::parse(&mut reader).expect_err(
            "total point count above MAX_TOTAL_POINTS must be rejected",
        );
        assert!(matches!(
            err,
            crate::parser::ParseError::UnexpectedPattern { .. }
        ));
    }

    #[test]
    fn parse_truncated_polygons_array_fails() {
        // NumberOfPolygons = 3 but only 1 entry follows.
        let mut data = Vec::new();
        data.extend_from_slice(&3_u16.to_le_bytes());
        data.extend_from_slice(&3_u16.to_le_bytes());
        let mut reader = &data[..];
        assert!(PolyPolygon::parse(&mut reader).is_err());
    }

    #[test]
    fn parse_truncated_points_array_fails() {
        // 1 polygon with 5 points, but only 2 PointS structures follow.
        let mut data = Vec::new();
        data.extend_from_slice(&1_u16.to_le_bytes());
        data.extend_from_slice(&5_u16.to_le_bytes());
        for v in [0_i16, 0, 10, 10] {
            data.extend_from_slice(&v.to_le_bytes());
        }
        let mut reader = &data[..];
        assert!(PolyPolygon::parse(&mut reader).is_err());
    }
}
