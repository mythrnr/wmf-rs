/// The BitmapV5Header Object contains information about the dimensions and
/// color format of a device-independent bitmap (DIB). It is an extension of the
/// BitmapV4Header Object. (Windows NT 3.1, Windows NT 3.5, Windows NT 3.51,
/// Windows 95, and Windows NT 4.0: This structure is not supported.)
#[derive(Clone, Debug)]
pub struct BitmapV5Header {
    /// BitmapV4Header (108 bytes): A BitmapV4Header Object, which defines
    /// properties of the DIB Object.
    ///
    /// When it is part of a BitmapV5Header, the ColorSpaceType field of a
    /// BitmapV4Header can be a logical color space value in the
    /// LogicalColorSpaceV5 Enumeration.
    pub bitmap_v4_header: crate::BitmapV4Header,
    /// Intent (4 bytes): A 32-bit unsigned integer that defines the rendering
    /// intent for the DIB. This MUST be a value defined in the
    /// GamutMappingIntent Enumeration.
    pub intent: crate::GamutMappingIntent,
    /// ProfileData (4 bytes): A 32-bit unsigned integer that defines the
    /// offset, in bytes, from the beginning of this structure to the start of
    /// the color profile data.
    ///
    /// If the color profile is embedded in the DIB, ProfileData is the offset
    /// to the actual color profile; if the color profile is linked,
    /// ProfileData is the offset to the null-terminated file name of the color
    /// profile. This MUST NOT be a Unicode string, but MUST be composed
    /// exclusively of characters from the Windows character set (code page
    /// 1252).
    ///
    /// If the ColorSpaceType field in the BitmapV4Header does not specify
    /// LCS_PROFILE_LINKED or LCS_PROFILE_EMBEDDED, the color profile data
    /// SHOULD be ignored.
    pub profile_data: u32,
    /// ProfileSize (4 bytes): A 32-bit unsigned integer that defines the size,
    /// in bytes, of embedded color profile data.
    pub profile_size: u32,
    /// Reserved (4 bytes): A 32-bit unsigned integer that is undefined and
    /// SHOULD be ignored.
    pub reserved: u32,
}

impl BitmapV5Header {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (
            (bitmap_v4_header, bitmap_v4_header_bytes),
            (intent, intent_bytes),
            (profile_data, profile_data_bytes),
            (profile_size, profile_size_bytes),
            (reserved, reserved_bytes),
        ) = (
            crate::BitmapV4Header::parse(buf)?,
            crate::GamutMappingIntent::parse(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
        );
        let consumed_bytes = profile_data_bytes
            + profile_size_bytes
            + reserved_bytes
            + bitmap_v4_header_bytes
            + intent_bytes;

        Ok((
            Self {
                bitmap_v4_header,
                intent,
                profile_data,
                profile_size,
                reserved,
            },
            consumed_bytes,
        ))
    }
}
