use crate::imports::*;

impl crate::parser::BitmapInfoHeader {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub(super) fn parse_as_info<R: crate::Read>(
        buf: &mut R,
        header_size: u32,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (width, width_bytes),
            (height, height_bytes),
            (planes, planes_bytes),
            (bit_count, bit_count_bytes),
            (compression, compression_bytes),
            (image_size, image_size_bytes),
            (x_pels_per_meter, x_pels_per_meter_bytes),
            (y_pels_per_meter, y_pels_per_meter_bytes),
            (color_used, color_used_bytes),
            (color_important, color_important_bytes),
        ) = (
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::BitCount::parse(buf)?,
            crate::parser::Compression::parse(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_i32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
            crate::parser::read_u32_from_le_bytes(buf)?,
        );
        let consumed_bytes = width_bytes
            + height_bytes
            + planes_bytes
            + image_size_bytes
            + x_pels_per_meter_bytes
            + y_pels_per_meter_bytes
            + color_used_bytes
            + color_important_bytes
            + bit_count_bytes
            + compression_bytes;

        if width <= 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The width field must be positive".to_owned(),
            });
        }

        if height == 0 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The height field must not be zero".to_owned(),
            });
        }

        if planes != 0x0001 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The planes field must be 0x01".to_owned(),
            });
        }

        Ok((
            Self::Info {
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
            },
            consumed_bytes,
        ))
    }
}
