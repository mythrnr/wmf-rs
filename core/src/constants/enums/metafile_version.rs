/// The MetafileVersion Enumeration defines values that specify support for
/// device-independent bitmaps (DIBs) in metafiles.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u16)]
pub enum MetafileVersion {
    /// DIBs are not supported.
    METAVERSION100 = 0x0100,
    /// DIBs are supported.
    METAVERSION300 = 0x0300,
}

crate::constants::impl_parser!(MetafileVersion, u16);
