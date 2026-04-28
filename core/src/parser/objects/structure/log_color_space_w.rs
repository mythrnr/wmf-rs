use crate::imports::*;

/// The LogColorSpaceW Object specifies a logical color space, which can be
/// defined by a color profile file with a name consisting of Unicode 16-bit
/// characters.
#[derive(Clone, Debug)]
pub struct LogColorSpaceW {
    /// A 32-bit unsigned integer that specifies the signature of color space
    /// objects. This MUST be set to the value 0x50534F43, which is the ASCII
    /// encoding of the string "PSOC".
    pub signature: u32,
    /// Version (4 bytes): A 32-bit unsigned integer that defines a version
    /// number; it MUST be 0x00000400.
    pub version: u32,
    /// Size (4 bytes): A 32-bit unsigned integer that defines the size of this
    /// object, in bytes.
    pub size: u32,
    /// ColorSpaceType (4 bytes): A 32-bit signed integer that specifies the
    /// color space type. It MUST be defined in the LogicalColorSpace
    /// Enumeration. If this value is LCS_sRGB or LCS_WINDOWS_COLOR_SPACE, the
    /// sRGB color space MUST be used.
    pub color_space_type: crate::parser::LogicalColorSpace,
    /// Intent (4 bytes): A 32-bit signed integer that defines the gamut
    /// mapping intent. It MUST be defined in the GamutMappingIntent
    /// Enumeration.
    pub intent: crate::parser::GamutMappingIntent,
    /// Endpoints (36 bytes): A CIEXYZTriple Object that defines the CIE
    /// chromaticity x, y, and z coordinates of the three colors that
    /// correspond to the RGB endpoints for the logical color space associated
    /// with the bitmap. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub endpoints: crate::parser::CIEXYZTriple,
    /// GammaRed (4 bytes): A 32-bit fixed point value that defines the toned
    /// response curve for red. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub gamma_red: u32,
    /// A 32-bit fixed point value that defines the toned response curve for
    /// green. If the ColorSpaceType field does not specify LCS_CALIBRATED_RGB,
    /// this field MUST be ignored.
    pub gamma_green: u32,
    /// GammaBlue (4 bytes): A 32-bit fixed point value that defines the toned
    /// response curve for blue. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub gamma_blue: u32,
    /// Filename (520 bytes): An optional, null-terminated Unicode UTF16-LE
    /// character string, which specifies the name of a file that contains a
    /// color profile. If a file name is specified, and the ColorSpaceType
    /// field is set to LCS_CALIBRATED_RGB, the other fields of this structure
    /// SHOULD be ignored.
    pub filename: Option<String>,
}

impl LogColorSpaceW {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let signature = read_field(buf, &mut consumed_bytes)?;
        let version = read_field(buf, &mut consumed_bytes)?;
        let size = read_field(buf, &mut consumed_bytes)?;
        let color_space_type = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::LogicalColorSpace::parse,
        )?;
        let intent = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::GamutMappingIntent::parse,
        )?;
        let endpoints = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::CIEXYZTriple::parse,
        )?;
        let gamma_red = read_field(buf, &mut consumed_bytes)?;
        let gamma_green = read_field(buf, &mut consumed_bytes)?;
        let gamma_blue = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq(
            "signature",
            signature,
            0x5053_4F43_u32,
        )?;
        crate::parser::ParseError::expect_eq(
            "version",
            version,
            0x0000_0400_u32,
        )?;

        let filename = if (size as usize).saturating_sub(consumed_bytes) >= 520
        {
            let bytes = read_bytes_field(buf, &mut consumed_bytes, 520)?;
            // Find NUL terminator in u16 units for UTF-16LE
            let len = bytes
                .chunks_exact(2)
                .position(|c| c == [0, 0])
                .map_or(bytes.len(), |pos| pos * 2);

            Some(crate::parser::objects::structure::utf16le_bytes_to_string(
                &bytes[..len],
            )?)
        } else {
            None
        };

        Ok((
            Self {
                signature,
                version,
                size,
                color_space_type,
                intent,
                endpoints,
                gamma_red,
                gamma_green,
                gamma_blue,
                filename,
            },
            consumed_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_payload(size: u32, with_filename: bool) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&0x5053_4F43_u32.to_le_bytes()); // signature
        data.extend_from_slice(&0x0000_0400_u32.to_le_bytes()); // version
        data.extend_from_slice(&size.to_le_bytes());
        data.extend_from_slice(&0x0000_0000_u32.to_le_bytes()); // LCS_CALIBRATED_RGB
        data.extend_from_slice(&0x0000_0001_u32.to_le_bytes()); // LCS_GM_BUSINESS
        data.extend_from_slice(&[0u8; 36]); // CIEXYZTriple
        data.extend_from_slice(&0_u32.to_le_bytes());
        data.extend_from_slice(&0_u32.to_le_bytes());
        data.extend_from_slice(&0_u32.to_le_bytes());
        if with_filename {
            data.extend_from_slice(&[0u8; 520]);
        }
        data
    }

    /// Same fixed prefix as `LogColorSpace`; the wide filename buffer
    /// is twice as long (UTF-16 LE, 260 chars * 2 bytes = 520).
    const FIXED_PREFIX: u32 = 68;
    const FILENAME_LEN: u32 = 520;

    #[test]
    fn parse_without_filename() {
        let data = build_payload(FIXED_PREFIX, false);
        let mut reader = &data[..];
        let (lcs, _) = LogColorSpaceW::parse(&mut reader).unwrap();
        assert!(lcs.filename.is_none());
    }

    #[test]
    fn parse_with_empty_filename() {
        // size = prefix + FILENAME_LEN ensures the filename block is read;
        // the 520-byte buffer is all-zero so the UTF-16 string is empty.
        let data = build_payload(FIXED_PREFIX + FILENAME_LEN, true);
        let mut reader = &data[..];
        let (lcs, _) = LogColorSpaceW::parse(&mut reader).unwrap();
        assert_eq!(lcs.filename.as_deref(), Some(""));
    }

    #[test]
    fn parse_rejects_invalid_version() {
        let mut data = build_payload(FIXED_PREFIX, false);
        data[4..8].copy_from_slice(&0xDEAD_BEEF_u32.to_le_bytes());
        let mut reader = &data[..];
        let err = LogColorSpaceW::parse(&mut reader).unwrap_err();
        assert!(matches!(err, crate::parser::ParseError::MismatchedField {
            field: "version",
            ..
        },));
    }
}
