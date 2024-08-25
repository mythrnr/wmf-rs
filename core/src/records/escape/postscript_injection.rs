impl crate::META_ESCAPE {
    pub(in crate::records::escape) fn parse_as_POSTSCRIPT_INJECTION<
        R: std::io::Read,
    >(
        buf: &mut R,
        mut record_size: crate::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::ParseError> {
        let (byte_count, byte_count_bytes) =
            crate::read_u16_from_le_bytes(buf)?;
        let (data, c) = crate::read_variable(buf, byte_count as usize)?;
        record_size.consume(byte_count_bytes + c);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::POSTSCRIPT_INJECTION {
            record_size,
            record_function,
            byte_count,
            data,
        })
    }
}
