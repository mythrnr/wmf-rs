/// The META_OFFSETCLIPRGN Record moves the clipping region in the playback
/// device context by the specified offsets.
#[derive(Clone, Debug)]
pub struct META_OFFSETCLIPRGN {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_OFFSETCLIPRGN.
    pub record_function: u16,
    /// YOffset (2 bytes): A 16-bit signed integer that defines the number of
    /// logical units to move up or down.
    pub y_offset: i16,
    /// XOffset (2 bytes): A 16-bit signed integer that defines the number of
    /// logical units to move left or right.
    pub x_offset: i16,
}

impl META_OFFSETCLIPRGN {
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
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_OFFSETCLIPRGN,
        )?;

        let ((y_offset, y_offset_bytes), (x_offset, x_offset_bytes)) = (
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(y_offset_bytes + x_offset_bytes);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, y_offset, x_offset })
    }
}
