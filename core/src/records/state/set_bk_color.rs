/// The META_SETBKCOLOR Record sets the background color in the playback device
/// context to a specified color, or to the nearest physical color if the device
/// cannot represent the specified color.
#[derive(Clone, Debug)]
pub struct META_SETBKCOLOR {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SETBKCOLOR.
    pub record_function: u16,
    /// ColorRef (4 bytes): A 32-bit ColorRef Object that defines the color
    /// value.
    pub color_ref: crate::ColorRef,
}

impl META_SETBKCOLOR {
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
            crate::RecordType::META_SETBKCOLOR,
        )?;

        let (color_ref, color_ref_bytes) = crate::ColorRef::parse(buf)?;
        record_size.consume(color_ref_bytes);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, color_ref })
    }
}
