/// The META_SAVEDC Record saves the playback device context for later
/// retrieval.
#[derive(Clone, Debug)]
pub struct META_SAVEDC {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SAVEDC.
    pub record_function: u16,
}

impl META_SAVEDC {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_SAVEDC,
        )?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function })
    }
}
