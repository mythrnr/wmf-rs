use crate::imports::*;

/// The Bitmap16 Object specifies information about the dimensions and color
/// format of a bitmap.
///
/// Bitmap16 object seems to be Windows DDB.
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
    pub bits_pixel: crate::parser::BitCount,
    /// Bits (variable): A variable length array of bytes that defines the
    /// bitmap pixel data. The length of this field in bytes can be computed as
    /// follows.
    ///
    /// ```text
    /// (((Width * BitsPixel + 15) >> 4) << 1) * Height
    /// ```
    pub bits: Vec<u8>,
}

impl core::fmt::Debug for Bitmap16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_with};

        let mut consumed_bytes: usize = 0;
        let mut bitmap =
            read_with(buf, &mut consumed_bytes, Self::parse_without_bits)?;
        let bits =
            read_bytes_field(buf, &mut consumed_bytes, bitmap.calc_length())?;

        bitmap.bits = bits;

        Ok((bitmap, consumed_bytes))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse_without_bits<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let typ = read_field(buf, &mut consumed_bytes)?;
        let width = read_field(buf, &mut consumed_bytes)?;
        let height = read_field(buf, &mut consumed_bytes)?;
        let width_bytes = read_field(buf, &mut consumed_bytes)?;
        let planes: u8 = read_field(buf, &mut consumed_bytes)?;
        let bits_pixel: u8 = read_field(buf, &mut consumed_bytes)?;

        if planes != 0x01 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The planes field must be 0x01".to_owned(),
            });
        }

        let (bits_pixel, _) = {
            let u16_bytes = u16::from(bits_pixel).to_le_bytes().to_vec();
            let mut b = &u16_bytes[..];

            crate::parser::BitCount::parse(&mut b)?
        };

        Ok((
            Self {
                typ,
                width,
                height,
                width_bytes,
                planes,
                bits_pixel,
                bits: vec![],
            },
            consumed_bytes,
        ))
    }

    pub fn calc_length(&self) -> usize {
        // Widen to i32 to prevent overflow.
        // Spec: (((Width * BitsPixel + 15) >> 4) << 1) * Height
        let w = i32::from(self.width);
        let bp = i32::from(u16::from(self.bits_pixel));
        let h = i32::from(self.height);

        let row_words = (w.saturating_mul(bp).saturating_add(15)) >> 4;
        let row_bytes = row_words << 1;

        row_bytes.saturating_mul(h).unsigned_abs() as usize
    }
}

impl From<Bitmap16> for crate::parser::DeviceIndependentBitmap {
    fn from(v: Bitmap16) -> Self {
        Self {
            dib_header_info: crate::parser::BitmapInfoHeader::Core(
                crate::parser::BitmapInfoHeaderCore {
                    header_size: 12,
                    width: v.width as u16,
                    height: v.height as u16,
                    planes: v.planes.into(),
                    bit_count: v.bits_pixel,
                },
            ),
            colors: crate::parser::Colors::Null,
            bitmap_buffer: crate::parser::BitmapBuffer { a_data: v.bits },
        }
    }
}
