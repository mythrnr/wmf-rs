impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_CLIP_TO_PATH<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let byte_count = read_field(buf, &mut record_size)?;
        let clip_function = read_with(
            buf,
            &mut record_size,
            crate::parser::PostScriptClipping::parse,
        )?;
        let reserved1: u16 = read_field(buf, &mut record_size)?;

        if byte_count != 0x0004 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The byte_count `{byte_count:#06X}` field must be `0x0004`",
                ),
            });
        }

        if reserved1 != 0x0000 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "The reserved1 `{reserved1:#06X}` field should be `0x0000`",
                ),
            });
        }

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::CLIP_TO_PATH {
            record_size,
            record_function,
            byte_count,
            clip_function,
            reserved1,
        })
    }
}
