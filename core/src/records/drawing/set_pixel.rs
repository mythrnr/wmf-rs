/// The META_SETPIXEL Record sets the pixel at the specified coordinates to the
/// specified color.
#[derive(Clone, Debug)]
pub struct META_SETPIXEL {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SETPIXEL.
    pub record_function: u16,
    /// ColorRef (4 bytes): A ColorRef Object that defines the color value.
    pub color_ref: crate::ColorRef,
    /// Y (2 bytes): A 16-bit signed integer that defines the y-coordinate, in
    /// logical units, of the point to be set.
    pub y: i16,
    /// X (2 bytes): A 16-bit signed integer that defines the x-coordinate, in
    /// logical units, of the point to be set.
    pub x: i16,
}

impl META_SETPIXEL {
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
            crate::RecordType::META_SETPIXEL,
        )?;

        let ((color_ref, color_ref_bytes), (y, y_bytes), (x, x_bytes)) = (
            crate::ColorRef::parse(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(color_ref_bytes + y_bytes + x_bytes);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, color_ref, y, x })
    }
}
