impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_GETCOLORTABLE<
        R: std::io::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        let ((byte_count, byte_count_bytes), (start, start_bytes)) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
        );
        record_size.consume(byte_count_bytes + start_bytes);
        let (_, c) = crate::parser::read_variable(buf, start as usize)?;
        record_size.consume(c);

        let (color_table_buffer, c) =
            crate::parser::read_variable(buf, (byte_count - start) as usize)?;
        record_size.consume(c);

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
