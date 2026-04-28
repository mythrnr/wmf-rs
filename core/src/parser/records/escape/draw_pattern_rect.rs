impl crate::parser::META_ESCAPE {
    pub(in crate::parser::records::escape) fn parse_as_DRAWPATTERNRECT<
        R: crate::Read,
    >(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let byte_count = read_field(buf, &mut record_size)?;
        let position =
            read_with(buf, &mut record_size, crate::parser::PointL::parse)?;
        let size =
            read_with(buf, &mut record_size, crate::parser::PointL::parse)?;
        let style = read_field(buf, &mut record_size)?;
        let pattern = read_field(buf, &mut record_size)?;

        crate::parser::ParseError::expect_eq("byte_count", byte_count, 0x0014)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self::DRAWPATTERNRECT {
            record_size,
            record_function,
            byte_count,
            position,
            size,
            style,
            pattern,
        })
    }
}
