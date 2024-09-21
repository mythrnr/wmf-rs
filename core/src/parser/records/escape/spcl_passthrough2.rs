impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_SPCLPASSTHROUGH2<
        R: std::io::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        let (
            (byte_count, byte_count_bytes),
            (reserved, reserved_bytes),
            (size, size_bytes),
        ) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
        );
        let (raw_data, c) = crate::parser::read_variable(buf, size as usize)?;
        record_size.consume(byte_count_bytes + reserved_bytes + size_bytes + c);

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
