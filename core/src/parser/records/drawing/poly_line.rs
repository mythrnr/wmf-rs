use crate::imports::*;

/// The META_POLYLINE Record draws a series of line segments by connecting the
/// points in the specified array.
#[derive(Clone, Debug)]
pub struct META_POLYLINE {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_POLYLINE.
    pub record_function: u16,
    /// NumberOfPoints (2 bytes): A 16-bit signed integer that defines the
    /// number of points in the array.
    pub number_of_points: i16,
    /// aPoints (variable): A NumberOfPoints array of 32-bit PointS Objects, in
    /// logical units.
    pub a_points: Vec<crate::parser::PointS>,
}

impl META_POLYLINE {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %crate::parser::HexU16(record_function),
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
            crate::parser::RecordType::META_POLYLINE,
        )?;

        let number_of_points = read_field(buf, &mut record_size)?;

        crate::parser::ParseError::expect_non_negative(
            "number_of_points",
            number_of_points,
        )?;

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
    fn parse_negative_number_of_points() {
        let payload = (-5_i16).to_le_bytes();
        let data = build_record(
            4,
            crate::parser::RecordType::META_POLYLINE as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        assert!(META_POLYLINE::parse(&mut reader, rs, rf).is_err());
    }

    #[test]
    fn parse_two_points() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&2_i16.to_le_bytes());
        payload.extend_from_slice(&10_i16.to_le_bytes());
        payload.extend_from_slice(&20_i16.to_le_bytes());
        payload.extend_from_slice(&30_i16.to_le_bytes());
        payload.extend_from_slice(&40_i16.to_le_bytes());
        let data = build_record(
            8,
            crate::parser::RecordType::META_POLYLINE as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_POLYLINE::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.number_of_points, 2);
        assert_eq!(record.a_points[0].x, 10);
        assert_eq!(record.a_points[1].y, 40);
    }
}
