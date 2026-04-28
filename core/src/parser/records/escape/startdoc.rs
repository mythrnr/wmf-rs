impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_STARTDOC<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field};

        let byte_count = read_field(buf, &mut record_size)?;

        crate::parser::ParseError::expect_le("byte_count", byte_count, 259)?;

        let doc_name =
            read_bytes_field(buf, &mut record_size, byte_count as usize)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::STARTDOC {
            record_size,
            record_function,
            byte_count,
            doc_name,
        })
    }
}
