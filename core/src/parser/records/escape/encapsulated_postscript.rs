impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_ENCAPSULATED_POSTSCRIPT<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field, read_with};

        let byte_count = read_field(buf, &mut record_size)?;
        let size = read_field(buf, &mut record_size)?;
        let version = read_field(buf, &mut record_size)?;
        let points =
            read_with(buf, &mut record_size, crate::parser::PointL::parse)?;

        if u32::from(byte_count) < size {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The byte_count field `{byte_count:#06X}` field must be \
                     greater than or equal to size field `{size:#06X}`",
                ),
            });
        }

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
