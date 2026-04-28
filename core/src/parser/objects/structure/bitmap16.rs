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

        crate::parser::ParseError::expect_eq("planes", planes, 0x01_u8)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a minimal Bitmap16 header (no `bits`) with valid planes and
    /// a bit_count that resolves to a known `BitCount` variant.
    fn build_header(width: i16, height: i16, bits_pixel: u8) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&0_i16.to_le_bytes()); // typ
        data.extend_from_slice(&width.to_le_bytes());
        data.extend_from_slice(&height.to_le_bytes());
        data.extend_from_slice(&width.to_le_bytes()); // width_bytes (matches `width` for simplicity)
        data.push(0x01); // planes (must be 0x01)
        data.push(bits_pixel);
        data
    }

    #[test]
    fn parse_without_bits_ok() {
        // bits_pixel = 0x01 -> BitCount::BI_BITCOUNT_1.
        let data = build_header(8, 4, 0x01);
        let mut reader = &data[..];
        let (b, consumed) = Bitmap16::parse_without_bits(&mut reader).unwrap();
        assert_eq!(b.width, 8);
        assert_eq!(b.height, 4);
        assert_eq!(b.planes, 0x01);
        assert!(
            matches!(b.bits_pixel, crate::parser::BitCount::BI_BITCOUNT_1,)
        );
        assert_eq!(consumed, 10);
    }

    #[test]
    fn parse_rejects_planes_not_one() {
        let mut data = build_header(1, 1, 0x01);
        data[8] = 0x02; // planes byte is at offset 8
        let mut reader = &data[..];
        let err = Bitmap16::parse_without_bits(&mut reader).unwrap_err();
        assert!(matches!(err, crate::parser::ParseError::MismatchedField {
            field: "planes",
            ..
        },));
    }

    #[test]
    fn parse_full_bitmap_includes_bits() {
        // BI_BITCOUNT_3 = 8 bpp, 2x2 image: row stride padded to a 16-bit
        // boundary makes 2 bytes/row, 4 bytes total.
        let mut data = build_header(2, 2, 0x08);
        data.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        let mut reader = &data[..];
        let (b, _) = Bitmap16::parse(&mut reader).unwrap();
        assert_eq!(b.bits, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn calc_length_matches_spec() {
        // 4x2 image at 8 bpp: row stride = ((4*8+15) >> 4) << 1 = 4 bytes,
        // total = 4 * 2 = 8.
        let bm = Bitmap16 {
            typ: 0,
            width: 4,
            height: 2,
            width_bytes: 4,
            planes: 1,
            bits_pixel: crate::parser::BitCount::BI_BITCOUNT_3,
            bits: vec![],
        };
        assert_eq!(bm.calc_length(), 8);
    }
}
