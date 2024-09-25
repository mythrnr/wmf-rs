use unicode_segmentation::UnicodeSegmentation;

/// The META_EXTTEXTOUT Record outputs text by using the font, background color,
/// and text color that are defined in the playback device context. Optionally,
/// dimensions can be provided for clipping, opaquing, or both.
#[derive(Clone, Debug)]
pub struct META_EXTTEXTOUT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_EXTTEXTOUT.
    pub record_function: u16,
    /// Y (2 bytes): A 16-bit signed integer that defines the y-coordinate, in
    /// logical units, where the text string is to be located.
    pub y: i16,
    /// X (2 bytes): A 16-bit signed integer that defines the x-coordinate, in
    /// logical units, where the text string is to be located.
    pub x: i16,
    /// StringLength (2 bytes): A 16-bit signed integer that defines the length
    /// of the string.
    pub string_length: i16,
    /// fwOpts (2 bytes): A 16-bit unsigned integer that defines the use of the
    /// application-defined rectangle. This member can be a combination of one
    /// or more values in the ExtTextOutOptions Flags.
    pub fw_opts: std::collections::BTreeSet<crate::parser::ExtTextOutOptions>,
    /// Rectangle (8 bytes): An optional 8-byte Rect Object. When either
    /// ETO_CLIPPED, ETO_OPAQUE, or both are specified, the rectangle defines
    /// the dimensions, in logical coordinates, used for clipping, opaquing, or
    /// both. When neither ETO_CLIPPED nor ETO_OPAQUE is specified, the
    /// coordinates in Rectangle are ignored.
    pub rectangle: Option<crate::parser::Rect>,
    /// String (variable): A variable-length string that specifies the text to
    /// be drawn. The string does not need to be null-terminated, because
    /// StringLength specifies the length of the string. If the length is odd,
    /// an extra byte is placed after it so that the following member (optional
    /// Dx) is aligned on a 16-bit boundary. The string will be decoded based
    /// on the font object currently selected into the playback device context.
    /// If a font matching the font object’s specification is not found, the
    /// decoding is undefined. If a matching font is found that matches the
    /// charset specified in the font object, the string should be decoded with
    /// the codepages in the following table.
    pub string: String,
    /// Dx (variable): An optional array of 16-bit signed integers that
    /// indicate the distance between origins of adjacent character cells. For
    /// example, Dx[i] logical units separate the origins of character cell i
    /// and character cell i + 1. If this field is present, there MUST be the
    /// same number of values as there are characters in the string.
    pub dx: Vec<i16>,
}

impl META_EXTTEXTOUT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
            ?charset,
        ),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
        charset: crate::parser::CharacterSet,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_EXTTEXTOUT,
        )?;

        let ((y, y_bytes), (x, x_bytes), (string_length, string_length_bytes)) = (
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(y_bytes + x_bytes + string_length_bytes);

        let fw_opts = {
            let (value, c) = crate::parser::read_u16_from_le_bytes(buf)?;
            record_size.consume(c);

            let mut fw_opts = std::collections::BTreeSet::new();

            for v in [
                crate::parser::ExtTextOutOptions::ETO_OPAQUE,
                crate::parser::ExtTextOutOptions::ETO_CLIPPED,
                crate::parser::ExtTextOutOptions::ETO_GLYPH_INDEX,
                crate::parser::ExtTextOutOptions::ETO_RTLREADING,
                crate::parser::ExtTextOutOptions::ETO_NUMERICSLOCAL,
                crate::parser::ExtTextOutOptions::ETO_NUMERICSLATIN,
                crate::parser::ExtTextOutOptions::ETO_PDY,
            ] {
                if value & (v as u16) == v as u16 {
                    fw_opts.insert(v);
                }
            }

            fw_opts
        };

        let rectangle = if fw_opts
            .contains(&crate::parser::ExtTextOutOptions::ETO_OPAQUE)
            || fw_opts.contains(&crate::parser::ExtTextOutOptions::ETO_CLIPPED)
        {
            let (v, c) = crate::parser::Rect::parse(buf)?;
            record_size.consume(c);

            Some(v)
        } else {
            None
        };

        let string = {
            let (bytes, c) =
                crate::parser::read_variable(buf, string_length as usize)?;
            record_size.consume(c);

            if charset == crate::parser::CharacterSet::SYMBOL_CHARSET {
                bytes
                    .into_iter()
                    .map(|v| {
                        (&*crate::parser::SYMBOL_CHARSET_TABLE).get(&v).cloned()
                    })
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect::<String>()
                    .replace("\0", "")
            } else {
                let encoding: &'static encoding_rs::Encoding = charset.into();
                let (cow, _, had_errors) = encoding.decode(&bytes);

                if had_errors {
                    return Err(crate::parser::ParseError::UnexpectedPattern {
                        cause: "cannot decode string".to_owned(),
                    });
                } else {
                    cow.replace("\0", "").to_owned()
                }
            }
        };

        // ignore odd bytes
        if string_length % 2 != 0 {
            let _ = crate::parser::read::<R, 1>(buf)?;
            record_size.consume(1);
        }

        let mut dx = vec![];

        if record_size.remaining() {
            for _ in 0..string.graphemes(true).count() {
                let (v, c) = crate::parser::read_i16_from_le_bytes(buf)?;

                record_size.consume(c);
                dx.push(v);
            }
        }

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self {
            record_size,
            record_function,
            y,
            x,
            string_length,
            fw_opts,
            rectangle,
            string,
            dx,
        })
    }
}

use std::{collections::HashMap, sync::LazyLock};

// via: https://en.wikipedia.org/wiki/Symbol_(typeface)
#[rustfmt::skip]
pub(in crate::parser) static SYMBOL_CHARSET_TABLE: LazyLock<HashMap<u8, char>> = LazyLock::new(|| {
    HashMap::from_iter([
        // 2x
        (0x20, ' '), (0x21, '!'), (0x22, '∀'), (0x23, '#'),
        (0x24, '∃'), (0x25, '%'), (0x26, '&'), (0x27, '∍'),
        (0x28, '('), (0x29, ')'), (0x2A, '*'), (0x2B, '+'),
        (0x2C, ','), (0x2D, '-'), (0x2E, '.'), (0x2F, '/'),
        // 3x
        (0x30, '0'), (0x31, '1'), (0x32, '2'), (0x33, '3'),
        (0x34, '4'), (0x35, '5'), (0x36, '6'), (0x37, '7'),
        (0x38, '8'), (0x39, '9'), (0x3A, ':'), (0x3B, ';'),
        (0x3C, '<'), (0x3D, '='), (0x3E, '>'), (0x3F, '?'),
        // 4x
        (0x40, '≅'), (0x41, 'Α'), (0x42, 'Β'), (0x43, 'Χ'),
        (0x44, 'Δ'), (0x45, 'Ε'), (0x46, 'Φ'), (0x47, 'Γ'),
        (0x48, 'Η'), (0x49, 'Ι'), (0x4A, 'ϑ'), (0x4B, 'Κ'),
        (0x4C, 'Λ'), (0x4D, 'Μ'), (0x4E, 'Ν'), (0x4F, 'Ο'),
        // 5x
        (0x50, 'Π'), (0x51, 'Θ'), (0x52, 'Ρ'), (0x53, 'Σ'),
        (0x54, 'Τ'), (0x55, 'Υ'), (0x56, 'ς'), (0x57, 'Ω'),
        (0x58, 'Ξ'), (0x59, 'Ψ'), (0x5A, 'Ζ'), (0x5B, '['),
        (0x5C, '∴'), (0x5D, ']'), (0x5E, '⊥'), (0x5F, '_'),
        // 6x
        (0x60, '‾'), (0x61, 'α'), (0x62, 'β'), (0x63, 'χ'),
        (0x64, 'δ'), (0x65, 'ε'), (0x66, 'φ'), (0x67, 'γ'),
        (0x68, 'η'), (0x69, 'ι'), (0x6A, 'ϕ'), (0x6B, 'κ'),
        (0x6C, 'λ'), (0x6D, 'μ'), (0x6E, 'ν'), (0x6F, 'ο'),
        // 7x
        (0x70, 'π'), (0x71, 'θ'), (0x72, 'ρ'), (0x73, 'σ'),
        (0x74, 'τ'), (0x75, 'υ'), (0x76, 'ϖ'), (0x77, 'ω'),
        (0x78, 'ξ'), (0x79, 'ψ'), (0x7A, 'ζ'), (0x7B, '{'),
        (0x7C, '|'), (0x7D, '}'), (0x7E, '~'),
        // Ax
        (0xA0, '€'), (0xA1, 'ϒ'), (0xA2, '′'), (0xA3, '≤'),
        (0xA4, '⁄'), (0xA5, '∞'), (0xA6, 'ƒ'), (0xA7, '♣'),
        (0xA8, '♦'), (0xA9, '♥'), (0xAA, '♠'), (0xAB, '↔'),
        (0xAC, '←'), (0xAD, '↑'), (0xAE, '→'), (0xAF, '↓'),
        // Bx
        (0xB0, '°'), (0xB1, '±'), (0xB2, '″'), (0xB3, '≥'),
        (0xB4, '×'), (0xB5, '∝'), (0xB6, '∂'), (0xB7, '•'),
        (0xB8, '÷'), (0xB9, '≠'), (0xBA, '≡'), (0xBB, '≈'),
        (0xBC, '…'), (0xBD, '⏐'), (0xBE, '⎯'), (0xBF, '↵'),
        // Cx
        (0xC0, 'ℵ'), (0xC1, 'ℑ'), (0xC2, 'ℜ'), (0xC3, '℘'),
        (0xC4, '⊗'), (0xC5, '⊕'), (0xC6, '∅'), (0xC7, '∩'),
        (0xC8, '∪'), (0xC9, '⊃'), (0xCA, '⊇'), (0xCB, '⊄'),
        (0xCC, '⊂'), (0xCD, '⊆'), (0xCE, '∈'), (0xCF, '∉'),
        //Dx
        (0xD0, '∠'), (0xD1, '∇'), (0xD2, '®'), (0xD3, '©'),
        (0xD4, '™'), (0xD5, '∏'), (0xD6, '√'), (0xD7, '⋅'),
        (0xD8, '¬'), (0xD9, '∧'), (0xDA, '∨'), (0xDB, '⇔'),
        (0xDC, '⇐'), (0xDD, '⇑'), (0xDE, '⇒'), (0xDF, '⇓'),
        // Ex
        (0xE0, '◊'), (0xE1, '⟨'), (0xE2, '®'), (0xE3, '©'),
        (0xE4, '™'), (0xE5, '∑'), (0xE6, '⎛'), (0xE7, '⎜'),
        (0xE8, '⎝'), (0xE9, '⎡'), (0xEA, '⎢'), (0xEB, '⎣'),
        (0xEC, '⎧'), (0xED, '⎨'), (0xEE, '⎩'), (0xEF, '⎪'),
        // Fx
        (0xF1, '⟩'), (0xF2, '∫'), (0xF3, '⌠'), (0xF4, '⎮'),
        (0xF5, '⌡'), (0xF6, '⎞'), (0xF7, '⎟'), (0xF8, '⎠'),
        (0xF9, '⎤'), (0xFA, '⎥'), (0xFB, '⎦'), (0xFC, '⎫'),
        (0xFD, '⎬'), (0xFE, '⎭'),
    ])
});
