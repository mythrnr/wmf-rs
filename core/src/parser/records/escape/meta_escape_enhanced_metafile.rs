impl crate::parser::META_ESCAPE {
    pub(super) fn parse_as_META_ESCAPE_ENHANCED_METAFILE<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field};

        let byte_count: u16 = read_field(buf, &mut record_size)?;
        let comment_identifier: u32 = read_field(buf, &mut record_size)?;
        let comment_type: u32 = read_field(buf, &mut record_size)?;
        let version: u32 = read_field(buf, &mut record_size)?;
        let checksum: u16 = read_field(buf, &mut record_size)?;
        let flags: u32 = read_field(buf, &mut record_size)?;
        let comment_record_count: u32 = read_field(buf, &mut record_size)?;
        let current_record_size: u32 = read_field(buf, &mut record_size)?;
        let remaining_bytes: u32 = read_field(buf, &mut record_size)?;
        let enhanced_metafile_data_size: u32 =
            read_field(buf, &mut record_size)?;

        let Some(expected_byte_count) =
            enhanced_metafile_data_size.checked_add(34)
        else {
            return Err(crate::parser::ParseError::FieldOutOfRange {
                field: "enhanced_metafile_data_size",
                actual: u64::from(enhanced_metafile_data_size),
                max: u64::from(u32::MAX - 34),
                width_bits: 32,
            });
        };

        // byte_count is u16 but the spec defines it relative to a u32
        // sum; widen for the comparison so width_bits stays 32 and the
        // hex display matches `expected_byte_count`'s width.
        crate::parser::ParseError::expect_eq(
            "byte_count",
            u32::from(byte_count),
            expected_byte_count,
        )?;
        crate::parser::ParseError::expect_eq(
            "comment_identifier",
            comment_identifier,
            0x4346_4D57_u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "comment_type",
            comment_type,
            0x0000_0001_u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "version",
            version,
            0x0001_0000_u32,
        )?;
        crate::parser::ParseError::expect_eq("flags", flags, 0x0000_0000_u32)?;
        crate::parser::ParseError::expect_le(
            "current_record_size",
            current_record_size,
            8192_u32,
        )?;

        let enhanced_metafile_data = read_bytes_field(
            buf,
            &mut record_size,
            enhanced_metafile_data_size as usize,
        )?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::META_ESCAPE_ENHANCED_METAFILE {
            record_size,
            record_function,
            byte_count,
            comment_identifier,
            comment_type,
            version,
            checksum,
            flags,
            comment_record_count,
            current_record_size,
            remaining_bytes,
            enhanced_metafile_data_size,
            enhanced_metafile_data,
        })
    }
}
