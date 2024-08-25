impl crate::META_ESCAPE {
    pub(in crate::records::escape) fn parse_as_GETCOLORTABLE<
        R: std::io::Read,
    >(
        buf: &mut R,
        mut record_size: crate::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::ParseError> {
        let ((byte_count, byte_count_bytes), (start, start_bytes)) = (
            crate::read_u16_from_le_bytes(buf)?,
            crate::read_u16_from_le_bytes(buf)?,
        );
        record_size.consume(byte_count_bytes + start_bytes);
        let (_, c) = crate::read_variable(buf, start as usize)?;
        record_size.consume(c);

        let (color_table_buffer, c) =
            crate::read_variable(buf, (byte_count - start) as usize)?;
        record_size.consume(c);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::GETCOLORTABLE {
            record_size,
            record_function,
            byte_count,
            start,
            color_table_buffer,
        })
    }
}
