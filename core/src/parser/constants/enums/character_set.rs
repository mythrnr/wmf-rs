/// The CharacterSet Enumeration defines the possible sets of character glyphs
/// that are defined in fonts for graphics output.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    strum::FromRepr,
    strum::EnumIter,
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

crate::parser::constants::impl_parser!(CharacterSet, u8);

impl From<CharacterSet> for &'static encoding_rs::Encoding {
    fn from(v: CharacterSet) -> Self {
        // Compile-time match avoids the Atomic / `static mut` /
        // BTreeMap allocation that an init-on-first-use cache would
        // require, and the previous implementation had a store-then-
        // write ordering that admitted a benign-looking but real data
        // race on multi-threaded hosts.
        codepage::to_encoding(codepage_for(v))
            .unwrap_or(encoding_rs::REPLACEMENT)
    }
}

/// Returns the Windows code page identifier for `charset`.
///
/// Anything not explicitly mapped (e.g. `DEFAULT_CHARSET`,
/// `MAC_CHARSET`) falls back to 1252, mirroring the previous
/// `unwrap_or(1252)` behavior.
const fn codepage_for(charset: CharacterSet) -> u16 {
    // via: https://en.wikipedia.org/wiki/Code_page
    match charset {
        CharacterSet::SYMBOL_CHARSET => 42,
        CharacterSet::SHIFTJIS_CHARSET => 932,
        CharacterSet::HANGUL_CHARSET => 949,
        CharacterSet::JOHAB_CHARSET => 1361,
        CharacterSet::GB2312_CHARSET => 936,
        CharacterSet::CHINESEBIG5_CHARSET => 950,
        CharacterSet::GREEK_CHARSET => 1253,
        CharacterSet::TURKISH_CHARSET => 1254,
        CharacterSet::VIETNAMESE_CHARSET => 1258,
        CharacterSet::HEBREW_CHARSET => 1255,
        CharacterSet::ARABIC_CHARSET => 1256,
        CharacterSet::BALTIC_CHARSET => 1257,
        CharacterSet::RUSSIAN_CHARSET => 1251,
        CharacterSet::THAI_CHARSET => 874,
        CharacterSet::EASTEUROPE_CHARSET => 1250,
        CharacterSet::OEM_CHARSET => 437,
        // ANSI_CHARSET, DEFAULT_CHARSET, MAC_CHARSET and any future
        // additions land here so the Windows-1252 fallback keeps
        // matching the original `unwrap_or(1252)`.
        _ => 1252,
    }
}

/// Maps a single byte through the Symbol-typeface table.
///
/// Bytes outside the table return `None`, which `bytes_into_utf8`
/// uses to drop them. The `match` lets the compiler emit a jump
/// table without any runtime `BTreeMap` lookup. A handful of
/// glyphs (e.g. ®, ©, ™) appear at two code points per the
/// Symbol-typeface table; the duplicate arms are kept verbatim
/// for readability rather than collapsed with `|`.
#[allow(clippy::match_same_arms)]
#[rustfmt::skip]
pub(in crate::parser) const fn map_symbol_charset(byte: u8) -> Option<char> {
    // via: https://en.wikipedia.org/wiki/Symbol_(typeface)
    match byte {
        // 2x
        0x20 => Some(' '), 0x21 => Some('!'), 0x22 => Some('∀'), 0x23 => Some('#'),
        0x24 => Some('∃'), 0x25 => Some('%'), 0x26 => Some('&'), 0x27 => Some('∍'),
        0x28 => Some('('), 0x29 => Some(')'), 0x2A => Some('*'), 0x2B => Some('+'),
        0x2C => Some(','), 0x2D => Some('-'), 0x2E => Some('.'), 0x2F => Some('/'),
        // 3x
        0x30 => Some('0'), 0x31 => Some('1'), 0x32 => Some('2'), 0x33 => Some('3'),
        0x34 => Some('4'), 0x35 => Some('5'), 0x36 => Some('6'), 0x37 => Some('7'),
        0x38 => Some('8'), 0x39 => Some('9'), 0x3A => Some(':'), 0x3B => Some(';'),
        0x3C => Some('<'), 0x3D => Some('='), 0x3E => Some('>'), 0x3F => Some('?'),
        // 4x
        0x40 => Some('≅'), 0x41 => Some('Α'), 0x42 => Some('Β'), 0x43 => Some('Χ'),
        0x44 => Some('Δ'), 0x45 => Some('Ε'), 0x46 => Some('Φ'), 0x47 => Some('Γ'),
        0x48 => Some('Η'), 0x49 => Some('Ι'), 0x4A => Some('ϑ'), 0x4B => Some('Κ'),
        0x4C => Some('Λ'), 0x4D => Some('Μ'), 0x4E => Some('Ν'), 0x4F => Some('Ο'),
        // 5x
        0x50 => Some('Π'), 0x51 => Some('Θ'), 0x52 => Some('Ρ'), 0x53 => Some('Σ'),
        0x54 => Some('Τ'), 0x55 => Some('Υ'), 0x56 => Some('ς'), 0x57 => Some('Ω'),
        0x58 => Some('Ξ'), 0x59 => Some('Ψ'), 0x5A => Some('Ζ'), 0x5B => Some('['),
        0x5C => Some('∴'), 0x5D => Some(']'), 0x5E => Some('⊥'), 0x5F => Some('_'),
        // 6x
        0x60 => Some('‾'), 0x61 => Some('α'), 0x62 => Some('β'), 0x63 => Some('χ'),
        0x64 => Some('δ'), 0x65 => Some('ε'), 0x66 => Some('φ'), 0x67 => Some('γ'),
        0x68 => Some('η'), 0x69 => Some('ι'), 0x6A => Some('ϕ'), 0x6B => Some('κ'),
        0x6C => Some('λ'), 0x6D => Some('μ'), 0x6E => Some('ν'), 0x6F => Some('ο'),
        // 7x
        0x70 => Some('π'), 0x71 => Some('θ'), 0x72 => Some('ρ'), 0x73 => Some('σ'),
        0x74 => Some('τ'), 0x75 => Some('υ'), 0x76 => Some('ϖ'), 0x77 => Some('ω'),
        0x78 => Some('ξ'), 0x79 => Some('ψ'), 0x7A => Some('ζ'), 0x7B => Some('{'),
        0x7C => Some('|'), 0x7D => Some('}'), 0x7E => Some('~'),
        // Ax
        0xA0 => Some('€'), 0xA1 => Some('ϒ'), 0xA2 => Some('′'), 0xA3 => Some('≤'),
        0xA4 => Some('⁄'), 0xA5 => Some('∞'), 0xA6 => Some('ƒ'), 0xA7 => Some('♣'),
        0xA8 => Some('♦'), 0xA9 => Some('♥'), 0xAA => Some('♠'), 0xAB => Some('↔'),
        0xAC => Some('←'), 0xAD => Some('↑'), 0xAE => Some('→'), 0xAF => Some('↓'),
        // Bx
        0xB0 => Some('°'), 0xB1 => Some('±'), 0xB2 => Some('″'), 0xB3 => Some('≥'),
        0xB4 => Some('×'), 0xB5 => Some('∝'), 0xB6 => Some('∂'), 0xB7 => Some('•'),
        0xB8 => Some('÷'), 0xB9 => Some('≠'), 0xBA => Some('≡'), 0xBB => Some('≈'),
        0xBC => Some('…'), 0xBD => Some('⏐'), 0xBE => Some('⎯'), 0xBF => Some('↵'),
        // Cx
        0xC0 => Some('ℵ'), 0xC1 => Some('ℑ'), 0xC2 => Some('ℜ'), 0xC3 => Some('℘'),
        0xC4 => Some('⊗'), 0xC5 => Some('⊕'), 0xC6 => Some('∅'), 0xC7 => Some('∩'),
        0xC8 => Some('∪'), 0xC9 => Some('⊃'), 0xCA => Some('⊇'), 0xCB => Some('⊄'),
        0xCC => Some('⊂'), 0xCD => Some('⊆'), 0xCE => Some('∈'), 0xCF => Some('∉'),
        // Dx
        0xD0 => Some('∠'), 0xD1 => Some('∇'), 0xD2 => Some('®'), 0xD3 => Some('©'),
        0xD4 => Some('™'), 0xD5 => Some('∏'), 0xD6 => Some('√'), 0xD7 => Some('⋅'),
        0xD8 => Some('¬'), 0xD9 => Some('∧'), 0xDA => Some('∨'), 0xDB => Some('⇔'),
        0xDC => Some('⇐'), 0xDD => Some('⇑'), 0xDE => Some('⇒'), 0xDF => Some('⇓'),
        // Ex
        0xE0 => Some('◊'), 0xE1 => Some('⟨'), 0xE2 => Some('®'), 0xE3 => Some('©'),
        0xE4 => Some('™'), 0xE5 => Some('∑'), 0xE6 => Some('⎛'), 0xE7 => Some('⎜'),
        0xE8 => Some('⎝'), 0xE9 => Some('⎡'), 0xEA => Some('⎢'), 0xEB => Some('⎣'),
        0xEC => Some('⎧'), 0xED => Some('⎨'), 0xEE => Some('⎩'), 0xEF => Some('⎪'),
        // Fx
        0xF1 => Some('⟩'), 0xF2 => Some('∫'), 0xF3 => Some('⌠'), 0xF4 => Some('⎮'),
        0xF5 => Some('⌡'), 0xF6 => Some('⎞'), 0xF7 => Some('⎟'), 0xF8 => Some('⎠'),
        0xF9 => Some('⎤'), 0xFA => Some('⎥'), 0xFB => Some('⎦'), 0xFC => Some('⎫'),
        0xFD => Some('⎬'), 0xFE => Some('⎭'),
        _ => None,
    }
}
