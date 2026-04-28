use crate::imports::*;

/// The Font object specifies the attributes of a logical font.
#[derive(Clone, Debug)]
pub struct Font {
    /// Height (2 bytes): A 16-bit signed integer that specifies the height, in
    /// logical units, of the font's character cell. The character height is
    /// computed as the character cell height minus the internal leading.
    ///
    /// For all height comparisons, the font mapper SHOULD find the largest
    /// physical font that does not exceed the requested size.
    ///
    /// All Windows versions: mapping the logical font size to the available
    /// physical fonts occurs the first time the logical font needs to be used
    /// in a drawing operation. For the MM_TEXT mapping mode, the following
    /// formula can be used to compute the height of a font with a specified
    /// point size.
    ///
    /// ```text
    /// Height = -MulDiv(PointSize, GetDeviceCaps(hDC, LOGPIXELSY), 72);
    /// ```
    pub height: i16,
    /// Width (2 bytes): A 16-bit signed integer that defines the average
    /// width, in logical units, of characters in the font. If Width is 0x0000,
    /// the aspect ratio of the device SHOULD be matched against the
    /// digitization aspect ratio of the available fonts to find the closest
    /// match, determined by the absolute value of the difference.
    pub width: i16,
    /// Escapement (2 bytes): A 16-bit signed integer that defines the angle,
    /// in tenths of degrees, between the escapement vector and the x-axis of
    /// the device. The escapement vector is parallel to the base line of a row
    /// of text.
    pub escapement: i16,
    /// Orientation (2 bytes): A 16-bit signed integer that defines the angle,
    /// in tenths of degrees, between each character's base line and the x-axis
    /// of the device.
    pub orientation: i16,
    /// Weight (2 bytes): A 16-bit signed integer that defines the weight of
    /// the font in the range 0 through 1000. For example, 400 is normal and
    /// 700 is bold. If this value is 0x0000, a default weight SHOULD be used.
    pub weight: i16,
    /// Italic (1 byte): A 8-bit Boolean value that specifies the italic
    /// attribute of the font.
    pub italic: bool,
    /// Underline (1 byte): An 8-bit Boolean value that specifies the underline
    /// attribute of the font.
    pub underline: bool,
    /// StrikeOut (1 byte): An 8-bit Boolean value that specifies the strikeout
    /// attribute of the font.
    pub strike_out: bool,
    /// CharSet (1 byte): An 8-bit unsigned integer that defines the character
    /// set. It SHOULD be set to a value in the CharacterSet Enumeration.
    ///
    /// The DEFAULT_CHARSET value MAY be used to allow the name and size of a
    /// font to fully describe the logical font. If the specified font name
    /// does not exist, a font in another character set MAY be substituted. The
    /// DEFAULT_CHARSET value is set to a value based on the current system
    /// locale. For example, when the system locale is United States, it is set
    /// to ANSI_CHARSET.
    ///
    /// If a typeface name in the FaceName field is specified, the CharSet
    /// value MUST match the character set of that typeface.
    pub charset: crate::parser::CharacterSet,
    /// OutPrecision (1 byte): An 8-bit unsigned integer that defines the
    /// output precision. The output precision defines how closely the output
    /// matches the requested font height, width, character orientation,
    /// escapement, pitch, and font type. It MUST be one of the values from the
    /// OutPrecision Enumeration.
    ///
    /// Applications can use the OUT_DEVICE_PRECIS, OUT_RASTER_PRECIS,
    /// OUT_TT_PRECIS, and OUT_PS_ONLY_PRECIS values to control how the font
    /// mapper selects a font when the operating system contains more than one
    /// font with a specified name. For example, if an operating system
    /// contains a font named "Symbol" in raster and TrueType forms, specifying
    /// OUT_TT_PRECIS forces the font mapper to select the TrueType version.
    /// Specifying OUT_TT_ONLY_PRECIS forces the font mapper to select a
    /// TrueType font, even if it substitutes a TrueType font of another name.
    pub out_precision: crate::parser::OutPrecision,
    /// ClipPrecision (1 byte): An 8-bit unsigned integer that defines the
    /// clipping precision. The clipping precision defines how to clip
    /// characters that are partially outside the clipping region. It MUST be a
    /// combination of one or more of the bit settings in the ClipPrecision
    /// Flags.
    pub clip_precision: crate::parser::ClipPrecision,
    /// Quality (1 byte): An 8-bit unsigned integer that defines the output
    /// quality. The output quality defines how carefully to attempt to match
    /// the logical font attributes to those of an actual physical font. It
    /// MUST be one of the values in the FontQuality Enumeration.
    pub quality: crate::parser::FontQuality,
    /// PitchAndFamily (1 byte): A PitchAndFamily Object that defines the pitch
    /// and the family of the font. Font families specify the look of fonts in
    /// a general way and are intended for specifying fonts when the exact
    /// typeface wanted is not available.
    pub pitch_and_family: crate::parser::PitchAndFamily,
    /// Facename (32 bytes): A null-terminated string of up to 32 8-bit Latin-1
    /// [ISO/IEC-8859-1] ANSI characters that specifies the typeface name of
    /// the font. Any characters following the terminating null are ignored.
    pub facename: String,
    /// Fallback facename: The facename interpreted according to the
    /// specified charset. This is extra field to help with font matching.
    pub fallback_facename: Vec<String>,
}

impl Font {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let height = read_field(buf, &mut consumed_bytes)?;
        let width = read_field(buf, &mut consumed_bytes)?;
        let escapement = read_field(buf, &mut consumed_bytes)?;
        let orientation = read_field(buf, &mut consumed_bytes)?;
        let weight = read_field(buf, &mut consumed_bytes)?;
        let italic_byte: u8 = read_field(buf, &mut consumed_bytes)?;
        let italic = italic_byte == 0x01;
        let underline_byte: u8 = read_field(buf, &mut consumed_bytes)?;
        let underline = underline_byte == 0x01;
        let strike_out_byte: u8 = read_field(buf, &mut consumed_bytes)?;
        let strike_out = strike_out_byte == 0x01;
        let charset = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::CharacterSet::parse,
        )?;
        let out_precision = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::OutPrecision::parse,
        )?;
        let clip_precision = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::ClipPrecision::parse,
        )?;
        let quality = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::FontQuality::parse,
        )?;
        let pitch_and_family = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::PitchAndFamily::parse,
        )?;

        let (facename, facename_as_charset) = {
            // The spec defines facename as a 32-byte field, but
            // real-world records may have fewer remaining bytes.
            // Read up to 32 bytes from the buffer. The fixed size
            // lets us keep this scratch on the stack.
            let mut bytes = [0u8; 32];
            let c = buf.read(&mut bytes).map_err(|err| {
                crate::parser::ParseError::UnexpectedPattern {
                    cause: format!("{err:?}").into(),
                }
            })?;
            consumed_bytes += c;

            // Find the position of the first null byte (0)
            let len = bytes[..c].iter().position(|&b| b == 0).unwrap_or(c);

            // Convert bytes to UTF-8 string from Latin-1
            let as_latin1 = crate::parser::bytes_into_utf8(
                &bytes[..len],
                crate::parser::CharacterSet::ANSI_CHARSET,
            )?;

            // Convert bytes to UTF-8 string from specified charset
            let as_charset =
                crate::parser::bytes_into_utf8(&bytes[..len], charset)?;

            (as_latin1, as_charset)
        };

        let mut fallback_facename = Vec::new();

        if facename != facename_as_charset {
            fallback_facename.push(facename_as_charset);
        }

        // If the facename refers to "Symbol" (case-insensitive),
        // promote the charset to SYMBOL_CHARSET. `contains_symbol`
        // walks the string once with ASCII case folding so we avoid
        // the per-call `to_ascii_lowercase` allocation that would
        // otherwise trigger for every CREATEFONTINDIRECT record.
        let charset = if contains_symbol(&facename)
            || fallback_facename.iter().any(|f| contains_symbol(f))
        {
            const SYMBOL: &str = "Symbol";
            if facename != SYMBOL
                && !fallback_facename.iter().any(|f| f == SYMBOL)
            {
                fallback_facename.push(SYMBOL.into());
            }

            crate::parser::CharacterSet::SYMBOL_CHARSET
        } else {
            charset
        };

        Ok((
            Self {
                height,
                width,
                escapement,
                orientation,
                weight,
                italic,
                underline,
                strike_out,
                charset,
                out_precision,
                clip_precision,
                quality,
                pitch_and_family,
                facename,
                fallback_facename,
            },
            consumed_bytes,
        ))
    }
}

/// Returns true when `s` contains the substring "symbol" under
/// ASCII case folding, without allocating an intermediate
/// lowercased copy.
fn contains_symbol(s: &str) -> bool {
    const NEEDLE: &[u8] = b"symbol";
    let bytes = s.as_bytes();
    if bytes.len() < NEEDLE.len() {
        return false;
    }
    bytes.windows(NEEDLE.len()).any(|w| w.eq_ignore_ascii_case(NEEDLE))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a 50-byte Font payload (18 byte header + 32 byte facename).
    fn build_font(facename: &[u8]) -> Vec<u8> {
        let mut data = Vec::new();
        // 18-byte header: 5 i16 fields + 3 u8 booleans + 5 u8 enums.
        data.extend_from_slice(&12_i16.to_le_bytes()); // height
        data.extend_from_slice(&0_i16.to_le_bytes()); // width
        data.extend_from_slice(&0_i16.to_le_bytes()); // escapement
        data.extend_from_slice(&0_i16.to_le_bytes()); // orientation
        data.extend_from_slice(&400_i16.to_le_bytes()); // weight
        data.push(0); // italic
        data.push(0); // underline
        data.push(0); // strike_out
        data.push(0); // charset (ANSI_CHARSET)
        data.push(0); // out_precision (OUT_DEFAULT_PRECIS)
        data.push(0); // clip_precision (CLIP_DEFAULT_PRECIS)
        data.push(0); // quality (DEFAULT_QUALITY)
        data.push(0); // pitch_and_family (FF_DONTCARE | DEFAULT_PITCH)
        // facename: up to 32 bytes, NUL-terminated.
        let mut name = vec![0u8; 32];
        let len = facename.len().min(31);
        name[..len].copy_from_slice(&facename[..len]);
        data.extend_from_slice(&name);
        data
    }

    #[test]
    fn parse_basic_font() {
        let data = build_font(b"Arial");
        let mut reader = &data[..];
        let (font, consumed) = Font::parse(&mut reader).unwrap();
        assert_eq!(font.height, 12);
        assert_eq!(font.weight, 400);
        assert!(!font.italic);
        assert_eq!(font.facename, "Arial");
        assert_eq!(consumed, 50);
    }

    /// When the facename starts with "Symbol", the charset is forced to
    /// SYMBOL_CHARSET regardless of what the header byte said.
    #[test]
    fn parse_symbol_facename_forces_symbol_charset() {
        let data = build_font(b"Symbol");
        let mut reader = &data[..];
        let (font, _) = Font::parse(&mut reader).unwrap();
        assert!(matches!(
            font.charset,
            crate::parser::CharacterSet::SYMBOL_CHARSET,
        ));
    }
}
