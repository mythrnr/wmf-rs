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
    pub string: String,
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
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
        charset: crate::parser::CharacterSet,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_TEXTOUT,
        )?;

        let (string_length, string_length_bytes) =
            crate::parser::read_i16_from_le_bytes(buf)?;
        record_size.consume(string_length_bytes);

        let string_len = if string_length % 2 == 0 {
            string_length
        } else {
            string_length + 1
        };
        let string = {
            let (bytes, c) =
                crate::parser::read_variable(buf, string_len as usize)?;
            record_size.consume(c);

            let encoding: &'static encoding_rs::Encoding = charset.into();
            let (cow, _, had_errors) = encoding.decode(&bytes);

            if had_errors {
                return Err(crate::parser::ParseError::UnexpectedPattern {
                    cause: "cannot decode string".to_owned(),
                });
            } else {
                cow.replace("\0", "").trim_end().to_owned()
            }
        };
        let ((y_start, y_start_bytes), (x_start, x_start_bytes)) = (
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(y_start_bytes + x_start_bytes);

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
}
