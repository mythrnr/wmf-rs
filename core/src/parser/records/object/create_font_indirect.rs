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
            record_function = %crate::parser::HexU16(record_function),
        ),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_bytes_field;

        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_CREATEFONTINDIRECT,
        )?;

        // Font consists of an 18-byte header + up to 32-byte facename.
        // The spec defines facename as a fixed 32-byte field, but
        // real-world WMF files often have shorter records where the
        // facename occupies only the remaining bytes.
        // The minimum required is 18 bytes (header fields only).
        // Use a bounded buffer to prevent Font::parse from consuming
        // bytes beyond the record boundary.
        const FONT_HEADER_SIZE: usize = 18;
        let remaining = record_size.remaining_bytes();

        if remaining < FONT_HEADER_SIZE {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "remaining bytes ({remaining}) is too small for Font \
                     (minimum {FONT_HEADER_SIZE} bytes)",
                )
                .into(),
            });
        }

        // Cap at 50 bytes (18 header + 32 facename) to guard
        // against corrupted record sizes.
        const FONT_MAX_SIZE: usize = 50;
        let read_len = core::cmp::min(remaining, FONT_MAX_SIZE);
        let b = read_bytes_field(buf, &mut record_size, read_len)?;

        let mut bounded = &b[..];
        let (font, _) = crate::parser::Font::parse(&mut bounded)?;

        // Only skip remaining bytes when record_size is reasonable.
        // Corrupted records may declare a huge size; attempting to
        // skip would fail or consume unrelated data.
        if record_size.remaining() && !record_size.is_overrun() {
            let skip = record_size.remaining_bytes();
            if skip <= FONT_MAX_SIZE {
                crate::parser::records::consume_remaining_bytes(
                    buf,
                    record_size,
                )?;
            }
        }

        Ok(Self { record_size, record_function, font })
    }
}
