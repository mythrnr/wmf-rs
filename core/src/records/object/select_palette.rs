/// The META_SELECTPALETTE Record defines the current logical palette with a
/// specified Palette Object.
#[derive(Clone, Debug)]
pub struct META_SELECTPALETTE {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SELECTPALETTE.
    pub record_function: u16,
    /// Palette (2 bytes): A 16-bit unsigned integer used to index into the WMF
    /// Object Table to get the Palette Object to be selected.
    pub palette: u16,
}

impl META_SELECTPALETTE {
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
            crate::RecordType::META_SELECTPALETTE,
        )?;

        let (palette, palette_bytes) = crate::read_u16_from_le_bytes(buf)?;
        record_size.consume(palette_bytes);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, palette })
    }
}
