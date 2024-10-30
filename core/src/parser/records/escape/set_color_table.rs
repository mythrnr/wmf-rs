impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_SETCOLORTABLE<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        let (byte_count, byte_count_bytes) =
            crate::parser::read_u16_from_le_bytes(buf)?;
        let (color_table, c) =
            crate::parser::read_variable(buf, byte_count as usize)?;
        record_size.consume(byte_count_bytes + c);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::SETCOLORTABLE {
            record_size,
            record_function,
            byte_count,
            color_table,
        })
    }
}
