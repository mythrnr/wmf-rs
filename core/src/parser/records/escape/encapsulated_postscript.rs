impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_ENCAPSULATED_POSTSCRIPT<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        let (
            (byte_count, byte_count_bytes),
            (size, size_bytes),
            (version, version_bytes),
            (points, points_bytes),
        ) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::PointL::parse(buf)?,
        );
        record_size.consume(
            byte_count_bytes + size_bytes + version_bytes + points_bytes,
        );

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
        let (data, c) =
            crate::parser::read_variable(buf, data_length as usize)?;
        record_size.consume(c);

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
