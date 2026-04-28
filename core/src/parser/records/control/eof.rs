/// The META_EOF Record indicates the end of the WMF metafile.
#[derive(Clone, Debug)]
pub struct META_EOF {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of 16-bit WORD structures, defined in [MS-DTYP] section 2.2.61, in the
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines the
    /// type of this record. For META_EOF, this value MUST be 0x0000, as
    /// specified in the RecordType Enumeration table.
    pub record_function: u16,
}

impl META_EOF {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        _buf: &mut R,
        record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        // word_size() returns usize; go through `u32::from(RecordSize)`
        // to keep the diagnostic width matching the on-wire field.
        crate::parser::ParseError::expect_eq(
            "record_size (words)",
            u32::from(record_size),
            3_u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "record_function",
            record_function,
            0x0000_u16,
        )?;

        Ok(Self { record_size, record_function })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let record_size = crate::parser::RecordSize::from_raw(3);
        let mut empty: &[u8] = &[];
        let record = META_EOF::parse(&mut empty, record_size, 0x0000).unwrap();
        assert_eq!(record.record_function, 0x0000);
    }

    #[test]
    fn parse_wrong_size() {
        let record_size = crate::parser::RecordSize::from_raw(4);
        let mut empty: &[u8] = &[];
        assert!(META_EOF::parse(&mut empty, record_size, 0x0000).is_err());
    }

    #[test]
    fn parse_wrong_function() {
        let record_size = crate::parser::RecordSize::from_raw(3);
        let mut empty: &[u8] = &[];
        assert!(META_EOF::parse(&mut empty, record_size, 0x0001).is_err());
    }
}
