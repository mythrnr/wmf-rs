impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_GETCOLORTABLE<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field};

        let byte_count = read_field(buf, &mut record_size)?;
        let start = read_field(buf, &mut record_size)?;

        crate::parser::ParseError::expect_le(
            "start (vs byte_count)",
            start,
            byte_count,
        )?;

        let _ = read_bytes_field(buf, &mut record_size, start as usize)?;

        let color_table_buffer = read_bytes_field(
            buf,
            &mut record_size,
            (byte_count - start) as usize,
        )?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::GETCOLORTABLE {
            record_size,
            record_function,
            byte_count,
            start,
            color_table_buffer,
        })
    }
}
