/// The META_INVERTREGION Record draws a region in which the colors are
/// inverted.
#[derive(Clone, Debug)]
pub struct META_INVERTREGION {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_INVERTREGION.
    pub record_function: u16,
    /// Region (2 bytes): A 16-bit unsigned integer used to index into the WMF
    /// Object Table to get the region to be inverted.
    pub region: u16,
}

impl META_INVERTREGION {
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
            crate::parser::RecordType::META_INVERTREGION,
        )?;

        let (region, region_bytes) = crate::parser::read_u16_from_le_bytes(buf)?;
        record_size.consume(region_bytes);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, region })
    }
}
