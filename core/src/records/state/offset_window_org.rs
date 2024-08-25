/// The META_OFFSETWINDOWORG Record moves the output window origin in the
/// playback device context by specified horizontal and vertical offsets.
#[derive(Clone, Debug)]
pub struct META_OFFSETWINDOWORG {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_OFFSETWINDOWORG.
    pub record_function: u16,
    /// YOffset (2 bytes): A 16-bit signed integer that defines the vertical
    /// offset, in device units.
    pub y_offset: i16,
    /// XOffset (2 bytes): A 16-bit signed integer that defines the horizontal
    /// offset, in device units.
    pub x_offset: i16,
}

impl META_OFFSETWINDOWORG {
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
            crate::RecordType::META_OFFSETWINDOWORG,
        )?;

        let ((y_offset, y_offset_bytes), (x_offset, x_offset_bytes)) = (
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(y_offset_bytes + x_offset_bytes);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, y_offset, x_offset })
    }
}
