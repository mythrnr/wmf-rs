impl crate::META_ESCAPE {
    pub(in crate::records::escape) fn parse_as_SETLINEJOIN<R: std::io::Read>(
        buf: &mut R,
        mut record_size: crate::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::ParseError> {
        let ((byte_count, byte_count_bytes), (join, join_bytes)) = (
            crate::read_u16_from_le_bytes(buf)?,
            crate::PostScriptJoin::parse(buf)?,
        );
        record_size.consume(byte_count_bytes + join_bytes);

        if byte_count != 0x0004 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: format!(
                    "The byte_count `{byte_count:#06X}` field must be `0x0004`",
                ),
            });
        }

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::SETLINEJOIN { record_size, record_function, byte_count, join })
    }
}
