use crate::imports::*;

/// The META_TEXTOUT Record outputs a character string at the specified location
/// by using the font, background color, and text color that are defined in the
/// playback device context.
#[derive(Clone, Debug)]
pub struct META_TEXTOUT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_TEXTOUT.
    pub record_function: u16,
    /// StringLength (2 bytes): A 16-bit signed integer that defines the length
    /// of the string, in bytes, pointed to by String.
    pub string_length: i16,
    /// String (variable): The size of this field MUST be a multiple of two. If
    /// StringLength is an odd number, then this field MUST be of a size
    /// greater than or equal to StringLength + 1. A variable-length string
    /// that specifies the text to be drawn. The string does not need to be
    /// null-terminated, because StringLength specifies the length of the
    /// string. The string is written at the location specified by the XStart
    /// and YStart fields.
    pub string: Vec<u8>,
    /// YStart (2 bytes): A 16-bit signed integer that defines the vertical
    /// (y-axis) coordinate, in logical units, of the point where drawing is to
    /// start.
    pub y_start: i16,
    /// XStart (2 bytes): A 16-bit signed integer that defines the horizontal
    /// (x-axis) coordinate, in logical units, of the point where drawing is to
    /// start.
    pub x_start: i16,
}

impl META_TEXTOUT {
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
            crate::parser::RecordType::META_TEXTOUT,
        )?;

        let (string_length, string_length_bytes) =
            crate::parser::read_i16_from_le_bytes(buf)?;
        record_size.consume(string_length_bytes);

        if string_length < 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "string_length must be non-negative, got {string_length}",
                ),
            });
        }

        // Pad to 2-byte boundary if string_length is odd.
        // Compute in usize to avoid i16 overflow (e.g. 32767 + 1).
        let string_len = string_length as usize + (string_length as usize % 2);

        let (
            (string, string_bytes),
            (y_start, y_start_bytes),
            (x_start, x_start_bytes),
        ) = (
            crate::parser::read_variable(buf, string_len)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(string_bytes + y_start_bytes + x_start_bytes);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self {
            record_size,
            record_function,
            string_length,
            string,
            y_start,
            x_start,
        })
    }

    /// Converts the string to UTF-8 using the specified character set.
    ///
    /// # Arguments
    ///
    /// - `charset` - The character set to use for conversion.
    ///
    /// # Returns
    ///
    /// A UTF-8 string, or `ParseError` if decoding fails.
    pub fn into_utf8(
        &self,
        charset: crate::parser::CharacterSet,
    ) -> Result<String, crate::parser::ParseError> {
        crate::parser::bytes_into_utf8(&self.string, charset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::records::test_helpers::*;

    #[test]
    fn parse_negative_string_length() {
        let payload = (-1_i16).to_le_bytes();
        let data = build_record(
            4,
            crate::parser::RecordType::META_TEXTOUT as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        assert!(META_TEXTOUT::parse(&mut reader, rs, rf).is_err());
    }

    #[test]
    fn parse_even_length_string() {
        let text = b"Hi";
        let mut payload = Vec::new();
        payload.extend_from_slice(&2_i16.to_le_bytes());
        payload.extend_from_slice(text);
        payload.extend_from_slice(&50_i16.to_le_bytes());
        payload.extend_from_slice(&100_i16.to_le_bytes());
        let data = build_record(
            7,
            crate::parser::RecordType::META_TEXTOUT as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_TEXTOUT::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.string_length, 2);
        assert_eq!(&record.string, text);
        assert_eq!(record.y_start, 50);
        assert_eq!(record.x_start, 100);
    }

    #[test]
    fn parse_odd_length_string() {
        let text = b"ABC";
        let mut payload = Vec::new();
        payload.extend_from_slice(&3_i16.to_le_bytes());
        payload.extend_from_slice(text);
        payload.push(0x00); // padding
        payload.extend_from_slice(&10_i16.to_le_bytes());
        payload.extend_from_slice(&20_i16.to_le_bytes());
        let data = build_record(
            8,
            crate::parser::RecordType::META_TEXTOUT as u16,
            &payload,
        );
        let (rs, rf, mut reader) = parse_record_header(&data);
        let record = META_TEXTOUT::parse(&mut reader, rs, rf).unwrap();
        assert_eq!(record.string_length, 3);
        assert_eq!(record.string.len(), 4);
        assert_eq!(&record.string[..3], text);
    }
}
