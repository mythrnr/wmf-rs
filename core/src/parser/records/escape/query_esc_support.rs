impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_QUERYESCSUPPORT<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        let ((byte_count, byte_count_bytes), (query, query_bytes)) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::MetafileEscapes::parse(buf)?,
        );
        record_size.consume(byte_count_bytes + query_bytes);

        if byte_count != 0x0002 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The byte_count `{byte_count:#06X}` field must be `0x0002`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::QUERYESCSUPPORT {
            record_size,
            record_function,
            byte_count,
            query,
        })
    }
}
