impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_ENCAPSULATED_POSTSCRIPT<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field, read_with};

        let byte_count: u16 = read_field(buf, &mut record_size)?;
        let size: u32 = read_field(buf, &mut record_size)?;
        let version: u32 = read_field(buf, &mut record_size)?;
        let points =
            read_with(buf, &mut record_size, crate::parser::PointL::parse)?;

        // byte_count is u16 in the wire format but `size` is u32.
        // Widen byte_count for the bound check so both operands share
        // the same width (and `width_bits` stays 32 for the diagnostic).
        crate::parser::ParseError::expect_le(
            "size (vs byte_count)",
            size,
            u32::from(byte_count),
        )?;

        let data_length = size
            - (u32::try_from(size_of::<crate::parser::PointL>())
                .expect("should be convert u32")
                + 4
                + 4);
        let data =
            read_bytes_field(buf, &mut record_size, data_length as usize)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::ENCAPSULATED_POSTSCRIPT {
            record_size,
            record_function,
            byte_count,
            size,
            version,
            points,
            data,
        })
    }
}
