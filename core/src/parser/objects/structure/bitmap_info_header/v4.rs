impl crate::parser::BitmapInfoHeader {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse_as_v4<R: crate::Read>(
        buf: &mut R,
        header_size: u32,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (header, header_bytes),
            (red_mask, red_mask_bytes),
            (green_mask, green_mask_bytes),
            (blue_mask, blue_mask_bytes),
            (alpha_mask, alpha_mask_bytes),
            (color_space_type, color_space_type_bytes),
            (endpoints, endpoints_bytes),
            (gamma_red, gamma_red_bytes),
            (gamma_green, gamma_green_bytes),
            (gamma_blue, gamma_blue_bytes),
        ) = (
            crate::parser::BitmapInfoHeader::parse_as_info(buf, header_size)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::LogicalColorSpace::parse(buf)?,
            crate::parser::CIEXYZTriple::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );
        let consumed_bytes = header_bytes
            + red_mask_bytes
            + green_mask_bytes
            + blue_mask_bytes
            + alpha_mask_bytes
            + color_space_type_bytes
            + endpoints_bytes
            + gamma_red_bytes
            + gamma_green_bytes
            + gamma_blue_bytes;

        let Self::Info {
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
        } = header
        else {
            unreachable!()
        };

        Ok((
            Self::V4 {
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
            },
            consumed_bytes,
        ))
    }
}
