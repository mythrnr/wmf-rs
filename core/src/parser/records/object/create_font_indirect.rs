/// The META_CREATEFONTINDIRECT Record creates a Font Object.
#[derive(Clone, Debug)]
pub struct META_CREATEFONTINDIRECT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_CREATEFONTINDIRECT.
    pub record_function: u16,
    /// Font (variable): Font Object data that defines the font to create.
    pub font: crate::parser::Font,
}

impl META_CREATEFONTINDIRECT {
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
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_CREATEFONTINDIRECT,
        )?;

        // Font consists of a fixed 19-byte header + up to 32 bytes facename.
        // Cap the read to a reasonable upper bound to avoid large
        // allocations, and read into a bounded buffer so that a crafted
        // too-small RecordSize cannot cause Font::parse to consume bytes
        // from subsequent records.
        const FONT_RECORD_MAX: usize = 256;
        let remaining = record_size.remaining_bytes();

        if remaining < 19 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "remaining bytes ({remaining}) is too small for Font \
                     (minimum 19 bytes)",
                ),
            });
        }

        let read_len = core::cmp::min(remaining, FONT_RECORD_MAX);
        let (b, c) = crate::parser::read_variable(buf, read_len)?;
        record_size.consume(c);

        let mut bounded = &b[..];
        let (font, _) = crate::parser::Font::parse(&mut bounded)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, font })
    }
}
