/// The META_FILLREGION Record fills a region using a specified brush.
#[derive(Clone, Debug)]
pub struct META_FILLREGION {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// record type. The lower byte MUST match the lower byte of the RecordType
    /// Enumeration table value META_FILLREGION.
    pub record_function: u16,
    /// Region (2 bytes): A 16-bit unsigned integer used to index into the WMF
    /// Object Table to get the region to be filled.
    pub region: u16,
    /// Brush (2 bytes): A 16-bit unsigned integer used to index into the WMF
    /// Object Table to get the brush to use for filling the region.
    pub brush: u16,
}

impl META_FILLREGION {
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
        use crate::parser::records::read_field;

        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_FILLREGION,
        )?;

        let region = read_field(buf, &mut record_size)?;
        let brush = read_field(buf, &mut record_size)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, region, brush })
    }
}
