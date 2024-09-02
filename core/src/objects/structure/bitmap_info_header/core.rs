impl crate::BitmapInfoHeader {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub(in crate::objects::structure::bitmap_info_header) fn parse_as_core<
        R: std::io::Read,
    >(
        buf: &mut R,
        header_size: u32,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (
            (width, width_bytes),
            (height, height_bytes),
            (planes, planes_bytes),
            (bit_count, bit_count_bytes),
        ) = (
            crate::read_u16_from_le_bytes(buf)?,
            crate::read_u16_from_le_bytes(buf)?,
            crate::read_u16_from_le_bytes(buf)?,
            crate::BitCount::parse(buf)?,
        );
        let consumed_bytes =
            width_bytes + height_bytes + planes_bytes + bit_count_bytes;

        if planes != 0x0001 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: "The planes field must be 0x01".to_owned(),
            });
        }

        if !matches!(
            bit_count,
            crate::BitCount::BI_BITCOUNT_1
                | crate::BitCount::BI_BITCOUNT_2
                | crate::BitCount::BI_BITCOUNT_3
                | crate::BitCount::BI_BITCOUNT_5
        ) {
            return Err(crate::ParseError::UnexpectedEnumValue {
                cause: format!(
                    "Invalid BitCount `{}` as Core type.",
                    bit_count as u16
                ),
            });
        }

        Ok((
            Self::Core { header_size, width, height, planes, bit_count },
            consumed_bytes,
        ))
    }
}
