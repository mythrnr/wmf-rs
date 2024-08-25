/// The LogicalColorSpace Enumeration specifies the type of color space.
/// (Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51: This functionality is
/// not supported.)
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum LogicalColorSpace {
    /// Color values are calibrated red green blue (RGB) values.
    LCS_CALIBRATED_RGB = 0x00000000,
    /// The value is an encoding of the ASCII characters "sRGB", and it
    /// indicates that the color values are sRGB values.
    LCS_sRGB = 0x73524742,
    /// The value is an encoding of the ASCII characters "Win ", including the
    /// trailing space, and it indicates that the color values are Windows
    /// default color space values.
    LCS_WINDOWS_COLOR_SPACE = 0x57696E20,
}

crate::constants::impl_parser!(LogicalColorSpace, u32);
