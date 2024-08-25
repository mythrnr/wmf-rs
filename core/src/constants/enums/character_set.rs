/// The CharacterSet Enumeration defines the possible sets of character glyphs
/// that are defined in fonts for graphics output.
#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum::FromRepr,
)]
#[repr(u8)]
pub enum CharacterSet {
    /// Specifies the English character set.
    ANSI_CHARSET = 0x00000000,
    /// Specifies a character set based on the current system locale; for
    /// example, when the system locale is United States English, the default
    /// character set is ANSI_CHARSET.
    DEFAULT_CHARSET = 0x00000001,
    /// Specifies a character set of symbols.
    SYMBOL_CHARSET = 0x00000002,
    /// This value is not supported.
    MAC_CHARSET = 0x0000004D,
    /// Specifies the Japanese character set.
    SHIFTJIS_CHARSET = 0x00000080,
    /// Also spelled "Hangeul". Specifies the Hangul Korean character set.
    HANGUL_CHARSET = 0x00000081,
    /// Also spelled "Johap". Specifies the Johab Korean character set.
    JOHAB_CHARSET = 0x00000082,
    /// Specifies the "simplified" Chinese character set for People's Republic
    /// of China.
    GB2312_CHARSET = 0x00000086,
    /// Specifies the "traditional" Chinese character set, used mostly in
    /// Taiwan and in the Hong Kong and Macao Special Administrative Regions.
    CHINESEBIG5_CHARSET = 0x00000088,
    /// Specifies the Greek character set.
    GREEK_CHARSET = 0x000000A1,
    /// Specifies the Turkish character set.
    TURKISH_CHARSET = 0x000000A2,
    /// Specifies the Vietnamese character set.
    VIETNAMESE_CHARSET = 0x000000A3,
    /// Specifies the Hebrew character set
    HEBREW_CHARSET = 0x000000B1,
    /// Specifies the Arabic character set
    ARABIC_CHARSET = 0x000000B2,
    /// Specifies the Baltic (Northeastern European) character set
    BALTIC_CHARSET = 0x000000BA,
    /// Specifies the Russian Cyrillic character set.
    RUSSIAN_CHARSET = 0x000000CC,
    /// Specifies the Thai character set.
    THAI_CHARSET = 0x000000DE,
    /// Specifies a Eastern European character set.
    EASTEUROPE_CHARSET = 0x000000EE,
    /// Specifies a mapping to one of the OEM code pages, according to the
    /// current system locale setting.
    OEM_CHARSET = 0x000000FF,
}

crate::constants::impl_parser!(CharacterSet, u8);

impl From<CharacterSet> for &'static encoding_rs::Encoding {
    fn from(v: CharacterSet) -> Self {
        match v {
            CharacterSet::DEFAULT_CHARSET | CharacterSet::ANSI_CHARSET => {
                encoding_rs::WINDOWS_1252
            }
            CharacterSet::ARABIC_CHARSET => encoding_rs::WINDOWS_1256,
            CharacterSet::BALTIC_CHARSET => encoding_rs::WINDOWS_1257,
            CharacterSet::CHINESEBIG5_CHARSET => encoding_rs::BIG5,
            CharacterSet::EASTEUROPE_CHARSET => encoding_rs::WINDOWS_1250,
            CharacterSet::GB2312_CHARSET => encoding_rs::GB18030,
            CharacterSet::GREEK_CHARSET => encoding_rs::WINDOWS_1253,
            CharacterSet::HANGUL_CHARSET => encoding_rs::EUC_KR,
            CharacterSet::HEBREW_CHARSET => encoding_rs::WINDOWS_1255,
            CharacterSet::RUSSIAN_CHARSET => encoding_rs::WINDOWS_1251,
            CharacterSet::SHIFTJIS_CHARSET => encoding_rs::SHIFT_JIS,
            CharacterSet::THAI_CHARSET => encoding_rs::WINDOWS_874,
            CharacterSet::TURKISH_CHARSET => encoding_rs::WINDOWS_1254,
            CharacterSet::VIETNAMESE_CHARSET => encoding_rs::WINDOWS_1258,
            // not defined in encoding_rs crate.
            CharacterSet::JOHAB_CHARSET
            | CharacterSet::MAC_CHARSET
            | CharacterSet::OEM_CHARSET
            | CharacterSet::SYMBOL_CHARSET => encoding_rs::WINDOWS_1252,
        }
    }
}
