impl crate::parser::BitmapInfoHeader {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse_as_v5<R: std::io::Read>(
        buf: &mut R,
        header_size: u32,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (header, header_bytes),
            (intent, intent_bytes),
            (profile_data, profile_data_bytes),
            (profile_size, profile_size_bytes),
            (reserved, reserved_bytes),
        ) = (
            crate::parser::BitmapInfoHeader::parse_as_v4(buf, header_size)?,
            crate::parser::GamutMappingIntent::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );
        let consumed_bytes = profile_data_bytes
            + profile_size_bytes
            + reserved_bytes
            + header_bytes
            + intent_bytes;

        let Self::V4 {
            header_size,
            width,
            height,
            planes,
            bit_count,
            compression,
            image_size,
            x_pels_per_meter,
            y_pels_per_meter,
            color_used,
            color_important,
            red_mask,
            green_mask,
            blue_mask,
            alpha_mask,
            color_space_type,
            endpoints,
            gamma_red,
            gamma_green,
            gamma_blue,
        } = header
        else {
            unreachable!()
        };

        Ok((
            Self::V5 {
                header_size,
                width,
                height,
                planes,
                bit_count,
                compression,
                image_size,
                x_pels_per_meter,
                y_pels_per_meter,
                color_used,
                color_important,
                red_mask,
                green_mask,
                blue_mask,
                alpha_mask,
                color_space_type,
                endpoints,
                gamma_red,
                gamma_green,
                gamma_blue,
                intent,
                profile_data,
                profile_size,
                reserved,
            },
            consumed_bytes,
        ))
    }
}
