/// The META_CREATEFONTINDIRECT Record creates a Font Object.
#[derive(Clone, Debug)]
pub struct META_CREATEFONTINDIRECT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_CREATEFONTINDIRECT.
    pub record_function: u16,
    /// Font (variable): Font Object data that defines the font to create.
    pub font: crate::Font,
}

impl META_CREATEFONTINDIRECT {
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
            crate::RecordType::META_CREATEFONTINDIRECT,
        )?;

        let (b, c) = crate::read_variable(buf, record_size.remaining_bytes())?;
        record_size.consume(c);

        let mut buffer = &b[..];
        let (font, _) = crate::Font::parse(&mut buffer)?;

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, font })
    }
}
