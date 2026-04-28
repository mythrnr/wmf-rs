impl crate::parser::META_ESCAPE {
    pub(super) fn parse_as_META_ESCAPE_ENHANCED_METAFILE<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field};

        let byte_count = read_field(buf, &mut record_size)?;
        let comment_identifier = read_field(buf, &mut record_size)?;
        let comment_type = read_field(buf, &mut record_size)?;
        let version = read_field(buf, &mut record_size)?;
        let checksum = read_field(buf, &mut record_size)?;
        let flags = read_field(buf, &mut record_size)?;
        let comment_record_count = read_field(buf, &mut record_size)?;
        let current_record_size = read_field(buf, &mut record_size)?;
        let remaining_bytes = read_field(buf, &mut record_size)?;
        let enhanced_metafile_data_size: u32 =
            read_field(buf, &mut record_size)?;

        let Some(expected_byte_count) =
            enhanced_metafile_data_size.checked_add(34)
        else {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "enhanced_metafile_data_size \
                     `{enhanced_metafile_data_size:#010X}` is too large",
                ),
            });
        };

        if u32::from(byte_count) != expected_byte_count {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The byte_count `{byte_count:#010X}` field must be same \
                     as `{expected_byte_count:#010X}`",
                ),
            });
        }

        if comment_identifier != 0x43464D57 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The comment_identifier `{comment_identifier:#010X}` \
                     field must be `0x43464D57`",
                ),
            });
        }

        if comment_type != 0x00000001 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The comment_type `{comment_type:#010X}` field must be \
                     `0x00000001`",
                ),
            });
        }

        if version != 0x00010000 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The version `{version:#010X}` field must be `0x00010000`"
                ),
            });
        }

        if flags != 0x00000000 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The flags `{flags:#010X}` field must be `0x00000000`",
                ),
            });
        }

        if current_record_size > 8192 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The current_record_size `{current_record_size}` field \
                     must be less than or equal to `8192`",
                ),
            });
        }

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
