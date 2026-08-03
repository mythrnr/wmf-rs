impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_SPCLPASSTHROUGH2<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field};

        let byte_count = read_field(buf, &mut record_size)?;
        let reserved = read_field(buf, &mut record_size)?;
        let size = read_field(buf, &mut record_size)?;
        let raw_data = read_bytes_field(buf, &mut record_size, size as usize)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::SPCLPASSTHROUGH2 {
            record_size,
            record_function,
            byte_count,
            reserved,
            size,
            raw_data,
        })
    }
}
