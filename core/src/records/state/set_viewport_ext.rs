/// The META_SETVIEWPORTEXT Record sets the horizontal and vertical extents of
/// the viewport in the playback device context.
#[derive(Clone, Debug)]
pub struct META_SETVIEWPORTEXT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SETVIEWPORTEXT.
    pub record_function: u16,
    /// Y (2 bytes): A 16-bit signed integer that defines the vertical extent
    /// of the viewport in device units.
    pub y: i16,
    /// X (2 bytes): A 16-bit signed integer that defines the horizontal extent
    /// of the viewport in device units.
    pub x: i16,
}

impl META_SETVIEWPORTEXT {
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
        mut record_size: crate::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::ParseError> {
        crate::records::check_lower_byte_matches(
            record_function,
            crate::RecordType::META_SETVIEWPORTEXT,
        )?;

        let ((y, y_bytes), (x, x_bytes)) = (
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(y_bytes + x_bytes);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, y, x })
    }
}
