/// The META_CREATEPALETTE Record creates a Palette Object.
#[derive(Clone, Debug)]
pub struct META_CREATEPALETTE {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_CREATEPALETTE.
    pub record_function: u16,
    /// Palette (variable): Palette Object data that defines the palette to
    /// create. The Start field in the Palette Object MUST be set to 0x0300.
    pub palette: crate::parser::Palette,
}

impl META_CREATEPALETTE {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %crate::parser::HexU16(record_function),
        ),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::read_with;

        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_CREATEPALETTE,
        )?;

        let palette =
            read_with(buf, &mut record_size, crate::parser::Palette::parse)?;

        crate::parser::ParseError::expect_eq(
            "palette.start",
            palette.start,
            0x0300,
        )?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, palette })
    }
}
