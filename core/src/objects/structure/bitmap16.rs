/// The Bitmap16 Object specifies information about the dimensions and color
/// format of a bitmap.
#[derive(Clone)]
pub struct Bitmap16 {
    /// Type (2 bytes): A 16-bit signed integer that defines the bitmap type.
    pub typ: i16,
    /// Width (2 bytes): A 16-bit signed integer that defines the width of the
    /// bitmap in pixels.
    pub width: i16,
    /// Height (2 bytes): A 16-bit signed integer that defines the height of
    /// the bitmap in scan lines.
    pub height: i16,
    /// WidthBytes (2 bytes): A 16-bit signed integer that defines the number
    /// of bytes per scan line.
    pub width_bytes: i16,
    /// Planes (1 byte): An 8-bit unsigned integer that defines the number of
    /// color planes in the bitmap. The value of this field MUST be 0x01.
    pub planes: u8,
    /// BitsPixel (1 byte): An 8-bit unsigned integer that defines the number
    /// of adjacent color bits on each plane.
    pub bits_pixel: u8,
    /// Bits (variable): A variable length array of bytes that defines the
    /// bitmap pixel data. The length of this field in bytes can be computed as
    /// follows.
    pub bits: Vec<u8>,
}

impl std::fmt::Debug for Bitmap16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmap16")
            .field("typ", &self.typ)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("width_bytes", &self.width_bytes)
            .field("planes", &self.planes)
            .field("bits_pixel", &self.bits_pixel)
            .field("bits", &format!("[u8; {}]", self.bits.len()))
            .finish()
    }
}

impl Bitmap16 {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (
            (typ, typ_bytes),
            (width, width_consumed_bytes),
            (height, height_bytes),
            (width_bytes, width_bytes_consumed_bytes),
            (planes, planes_bytes),
            (bits_pixel, bits_pixel_bytes),
        ) = (
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_i16_from_le_bytes(buf)?,
            crate::read_u8_from_le_bytes(buf)?,
            crate::read_u8_from_le_bytes(buf)?,
        );
        let mut consumed_bytes = typ_bytes
            + width_consumed_bytes
            + height_bytes
            + width_bytes_consumed_bytes
            + planes_bytes
            + bits_pixel_bytes;

        if planes != 0x01 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: "The planes field must be 0x01".to_owned(),
            });
        }

        let bits_length =
            (((width * i16::from(bits_pixel) + 15) >> 4) << 1) * height;
        let (bits, bits_bytes) =
            crate::read_variable(buf, bits_length as usize)?;
        consumed_bytes += bits_bytes;

        Ok((
            Self { typ, width, height, width_bytes, planes, bits_pixel, bits },
            consumed_bytes,
        ))
    }
}
