use crate::imports::*;

/// The META_POLYGON Record paints a polygon consisting of two or more vertices
/// connected by straight lines. The polygon is outlined by using the pen and
/// filled by using the brush and polygon fill mode that are defined in the
/// playback device context.
#[derive(Clone, Debug)]
pub struct META_POLYGON {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_POLYGON.
    pub record_function: u16,
    /// NumberOfPoints (2 bytes): A 16-bit signed integer that defines the
    /// number of points in the array. This value must be greater than or equal
    /// to 2.
    pub number_of_points: i16,
    /// aPoints (variable): A NumberOfPoints array of 32-bit PointS Objects, in
    /// logical units.
    pub a_points: Vec<crate::parser::PointS>,
}

impl META_POLYGON {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_POLYGON,
        )?;

        let number_of_points = read_field(buf, &mut record_size)?;

        // The spec requires number_of_points >= 2, but real-world
        // WMF files may contain degenerate polygons (0 or 1 points).
        // Treat these as a no-op; only reject negative values.
        if number_of_points < 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "number_of_points must be >= 0, got {number_of_points}",
                ),
            });
        }

        let mut a_points = Vec::with_capacity(number_of_points as usize);

        for _ in 0..number_of_points {
            a_points.push(read_with(
                buf,
                &mut record_size,
                crate::parser::PointS::parse,
            )?);
        }

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, number_of_points, a_points })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::records::test_helpers::*;

    #[test]
    fn parse_rejects_negative_number_of_points() {
        let payload = (-1_i16).to_le_bytes();
        let data = build_record(
            4,
            crate::parser::RecordType::META_POLYGON as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        assert!(META_POLYGON::parse(&mut reader, rs, rf).is_err());
    }

    #[test]
    fn parse_accepts_zero_points() {
        let payload = 0_i16.to_le_bytes();
        let data = build_record(
            4,
            crate::parser::RecordType::META_POLYGON as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_POLYGON::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.number_of_points, 0);
        assert!(record.a_points.is_empty());
    }

    #[test]
    fn parse_single_point() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&1_i16.to_le_bytes());
        // Single PointS (x = 10, y = 20)
        payload.extend_from_slice(&10_i16.to_le_bytes());
        payload.extend_from_slice(&20_i16.to_le_bytes());
        let data = build_record(
            6,
            crate::parser::RecordType::META_POLYGON as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_POLYGON::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.number_of_points, 1);
        assert_eq!(record.a_points.len(), 1);
        assert_eq!(record.a_points[0].x, 10);
        assert_eq!(record.a_points[0].y, 20);
    }

    #[test]
    fn parse_triangle() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&3_i16.to_le_bytes());
        for (x, y) in [(0_i16, 0_i16), (100, 0), (50, 100)] {
            payload.extend_from_slice(&x.to_le_bytes());
            payload.extend_from_slice(&y.to_le_bytes());
        }
        let data = build_record(
            10,
            crate::parser::RecordType::META_POLYGON as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_POLYGON::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.number_of_points, 3);
        assert_eq!(record.a_points.len(), 3);
        assert_eq!(record.a_points[2].x, 50);
        assert_eq!(record.a_points[2].y, 100);
    }

    #[test]
    fn parse_insufficient_points() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&3_i16.to_le_bytes());
        payload.extend_from_slice(&10_i16.to_le_bytes());
        payload.extend_from_slice(&20_i16.to_le_bytes());
        let data = build_record(
            10,
            crate::parser::RecordType::META_POLYGON as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        assert!(META_POLYGON::parse(&mut reader, rs, rf).is_err());
    }
}
