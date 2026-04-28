/// The META_RECTANGLE Record paints a rectangle. The rectangle is outlined by
/// using the pen and filled by using the brush that are defined in the playback
/// device context.
#[derive(Clone, Debug)]
pub struct META_RECTANGLE {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_RECTANGLE.
    pub record_function: u16,
    /// BottomRect (2 bytes): A 16-bit signed integer that defines the
    /// y-coordinate, in logical units, of the lower-right corner of the
    /// rectangle.
    pub bottom_rect: i16,
    /// RightRect (2 bytes): A 16-bit signed integer that defines the
    /// x-coordinate, in logical units, of the lower-right corner of the
    /// rectangle.
    pub right_rect: i16,
    /// TopRect (2 bytes): A 16-bit signed integer that defines the
    /// y-coordinate, in logical units, of the upper-left corner of the
    /// rectangle.
    pub top_rect: i16,
    /// LeftRect (2 bytes): A 16-bit signed integer that defines the
    /// x-coordinate, in logical units, of the upper-left corner of the
    /// rectangle.
    pub left_rect: i16,
}

impl META_RECTANGLE {
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
        use crate::parser::records::read_field;

        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_RECTANGLE,
        )?;

        let bottom_rect = read_field(buf, &mut record_size)?;
        let right_rect = read_field(buf, &mut record_size)?;
        let top_rect = read_field(buf, &mut record_size)?;
        let left_rect = read_field(buf, &mut record_size)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self {
            record_size,
            record_function,
            bottom_rect,
            right_rect,
            top_rect,
            left_rect,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{imports::*, parser::records::test_helpers::*};

    #[test]
    fn parse_ok() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&200_i16.to_le_bytes());
        payload.extend_from_slice(&300_i16.to_le_bytes());
        payload.extend_from_slice(&10_i16.to_le_bytes());
        payload.extend_from_slice(&20_i16.to_le_bytes());
        let data = build_record(
            7,
            crate::parser::RecordType::META_RECTANGLE as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_RECTANGLE::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.bottom_rect, 200);
        assert_eq!(record.right_rect, 300);
        assert_eq!(record.top_rect, 10);
        assert_eq!(record.left_rect, 20);
    }

    #[test]
    fn parse_insufficient_buffer() {
        let payload = [200_i16.to_le_bytes(), 300_i16.to_le_bytes()].concat();
        let data = build_record(
            7,
            crate::parser::RecordType::META_RECTANGLE as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        assert!(META_RECTANGLE::parse(&mut reader, rs, rf).is_err());
    }
}
