/// The LogicalColorSpaceV5 Enumeration is used to specify where to find color
/// profile information for a DeviceIndependentBitmap (DIB) Object that has a
/// header of type BitmapV5Header Object.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u32)]
pub enum LogicalColorSpaceV5 {
    /// The value consists of the string "LINK" from the Windows character set
    /// (code page 1252). It indicates that the color profile MUST be linked
    /// with the DIB Object.
    LCS_PROFILE_LINKED = 0x4C494E4B,
    /// The value consists of the string "MBED" from the Windows character set
    /// (code page 1252). It indicates that the color profile MUST be embedded
    /// in the DIB Object.
    LCS_PROFILE_EMBEDDED = 0x4D424544,
}

crate::constants::impl_parser!(LogicalColorSpaceV5, u32);
