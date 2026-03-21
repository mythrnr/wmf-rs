/// The ColorRef Object defines the RGB color.
#[derive(Clone, Debug)]
pub struct ColorRef {
    /// Red (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of red.
    pub red: u8,
    /// Green (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of green.
    pub green: u8,
    /// Blue (1 byte): An 8-bit unsigned integer that defines the relative
    /// intensity of blue.
    pub blue: u8,
    /// Reserved (1 byte): An 8-bit unsigned integer that MUST be 0x00.
    pub reserved: u8,
}

impl ColorRef {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (
            (red, red_bytes),
            (green, green_bytes),
            (blue, blue_bytes),
            (mut reserved, reserved_bytes),
        ) = (
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
        );

        if reserved != 0x00 {
            warn!(
                reserved = %format!("{reserved:#04X}"),
                "The reserved field is replaced by 0x00; MS-WMF states that \
                this field MUST be 0x00",
            );

            reserved = 0x00;
        }

        Ok((
            Self { red, green, blue, reserved },
            red_bytes + green_bytes + blue_bytes + reserved_bytes,
        ))
    }
}

impl ColorRef {
    pub fn black() -> Self {
        Self { red: 0, green: 0, blue: 0, reserved: 0 }
    }

    pub fn white() -> Self {
        Self { red: 255, green: 255, blue: 255, reserved: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let data = [0xFF, 0x80, 0x00, 0x00];
        let mut reader = &data[..];
        let (color, consumed) = ColorRef::parse(&mut reader).unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 0);
        assert_eq!(color.reserved, 0);
        assert_eq!(consumed, 4);
    }

    #[test]
    fn parse_reserved_nonzero_is_replaced() {
        let data = [0x10, 0x20, 0x30, 0xFF];
        let mut reader = &data[..];
        let (color, _) = ColorRef::parse(&mut reader).unwrap();
        assert_eq!(
            color.reserved, 0x00,
            "reserved field should be replaced with 0x00"
        );
    }
}
