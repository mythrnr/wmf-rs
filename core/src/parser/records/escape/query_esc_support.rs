impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_QUERYESCSUPPORT<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let byte_count = read_field(buf, &mut record_size)?;
        let query = read_with(
            buf,
            &mut record_size,
            crate::parser::MetafileEscapes::parse,
        )?;

        crate::parser::ParseError::expect_eq("byte_count", byte_count, 0x0002)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::QUERYESCSUPPORT {
            record_size,
            record_function,
            byte_count,
            query,
        })
    }
}
