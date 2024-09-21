impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_SETLINECAP<R: std::io::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        let ((byte_count, byte_count_bytes), (cap, cap_bytes)) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::PostScriptCap::parse(buf)?,
        );
        record_size.consume(byte_count_bytes + cap_bytes);

        if byte_count != 0x0004 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The byte_count `{byte_count:#06X}` field must be `0x0004`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::SETLINECAP { record_size, record_function, byte_count, cap })
    }
}
