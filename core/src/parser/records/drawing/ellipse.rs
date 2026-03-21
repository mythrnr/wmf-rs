/// The META_ELLIPSE Record draws an ellipse. The center of the ellipse is the
/// center of the specified bounding rectangle. The ellipse is outlined by using
/// the pen and is filled by using the brush; these are defined in the playback
/// device context.
#[derive(Clone, Debug)]
pub struct META_ELLIPSE {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_ELLIPSE.
    pub record_function: u16,
    /// BottomRect (2 bytes): A 16-bit signed integer that defines the
    /// y-coordinate, in logical units, of the lower-right corner of the
    /// bounding rectangle.
    pub bottom_rect: i16,
    /// RightRect (2 bytes): A 16-bit signed integer that defines the
    /// x-coordinate, in logical units, of the lower-right corner of the
    /// bounding rectangle.
    pub right_rect: i16,
    /// TopRect (2 bytes): A 16-bit signed integer that defines the
    /// y-coordinate, in logical units, of the upper-left corner of the
    /// bounding rectangle.
    pub top_rect: i16,
    /// LeftRect (2 bytes): A 16-bit signed integer that defines the
    /// x-coordinate, in logical units, of the upper-left corner of the
    /// bounding rectangle.
    pub left_rect: i16,
}

impl META_ELLIPSE {
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
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_ELLIPSE,
        )?;

        let (
            (bottom_rect, bottom_rect_bytes),
            (right_rect, right_rect_bytes),
            (top_rect, top_rect_bytes),
            (left_rect, left_rect_bytes),
        ) = (
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(
            bottom_rect_bytes
                + right_rect_bytes
                + top_rect_bytes
                + left_rect_bytes,
        );

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
        payload.extend_from_slice(&150_i16.to_le_bytes());
        payload.extend_from_slice(&200_i16.to_le_bytes());
        payload.extend_from_slice(&50_i16.to_le_bytes());
        payload.extend_from_slice(&60_i16.to_le_bytes());
        let data = build_record(
            7,
            crate::parser::RecordType::META_ELLIPSE as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_ELLIPSE::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.bottom_rect, 150);
        assert_eq!(record.right_rect, 200);
        assert_eq!(record.top_rect, 50);
        assert_eq!(record.left_rect, 60);
    }
}
