use crate::imports::*;

/// The LogColorSpace Object specifies a logical color space for the playback
/// device context, which can be the name of a color profile in ASCII
/// characters.
#[derive(Clone, Debug)]
pub struct LogColorSpace {
    /// Signature (4 bytes): A 32-bit unsigned integer that specifies the
    /// signature of color space objects; it MUST be set to the value
    /// 0x50534F43, which is the ASCII encoding of the string "PSOC".
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
    /// GammaGreen (4 bytes): A 32-bit fixed point value that defines the toned
    /// response curve for green. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub gamma_green: u32,
    /// GammaBlue (4 bytes): A 32-bit fixed point value that defines the toned
    /// response curve for blue. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub gamma_blue: u32,
    /// Filename (260 bytes): An optional, ASCII charactger string that
    /// specifies the name of a file that contains a color profile. If a file
    /// name is specified, and the ColorSpaceType field is set to
    /// LCS_CALIBRATED_RGB, the other fields of this structure SHOULD be
    /// ignored.
    pub filename: Option<String>,
}

impl LogColorSpace {
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

        let filename = if (size as usize).saturating_sub(consumed_bytes) >= 260
        {
            let bytes = read_bytes_field(buf, &mut consumed_bytes, 260)?;

            // Strip trailing NUL padding from fixed-length buffer
            let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());

            Some(String::from_utf8_lossy(&bytes[..end]).to_string())
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

    /// Builds a synthetic LogColorSpace payload with valid signature
    /// and version. Reused across the success cases below.
    fn build_payload(size: u32, with_filename: bool) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&0x5053_4F43_u32.to_le_bytes()); // signature
        data.extend_from_slice(&0x0000_0400_u32.to_le_bytes()); // version
        data.extend_from_slice(&size.to_le_bytes());
        data.extend_from_slice(&0x0000_0000_u32.to_le_bytes()); // LCS_CALIBRATED_RGB
        data.extend_from_slice(&0x0000_0001_u32.to_le_bytes()); // LCS_GM_BUSINESS
        // Endpoints: 9 u32 zeros (CIEXYZTriple).
        data.extend_from_slice(&[0u8; 36]);
        data.extend_from_slice(&0_u32.to_le_bytes()); // gamma_red
        data.extend_from_slice(&0_u32.to_le_bytes()); // gamma_green
        data.extend_from_slice(&0_u32.to_le_bytes()); // gamma_blue
        if with_filename {
            data.extend_from_slice(&[0u8; 260]);
        }
        data
    }

    /// Fixed prefix size is 68 bytes (sig 4 + ver 4 + size 4 +
    /// color_space_type 4 + intent 4 + endpoints 36 + 3 * gamma 4); the
    /// optional 260-byte filename trails it for a total of 328 bytes.
    const FIXED_PREFIX: u32 = 68;
    const FILENAME_LEN: u32 = 260;

    #[test]
    fn parse_without_filename() {
        // size = FIXED_PREFIX -> filename block is not read.
        let data = build_payload(FIXED_PREFIX, false);
        let mut reader = &data[..];
        let (lcs, _) = LogColorSpace::parse(&mut reader).unwrap();
        assert_eq!(lcs.signature, 0x5053_4F43);
        assert_eq!(lcs.version, 0x0000_0400);
        assert!(lcs.filename.is_none());
    }

    #[test]
    fn parse_with_filename() {
        // size = FIXED_PREFIX + FILENAME_LEN makes the filename block
        // readable; the 260-byte buffer is all zeros so the resulting
        // filename is an empty string (everything before the first NUL).
        let data = build_payload(FIXED_PREFIX + FILENAME_LEN, true);
        let mut reader = &data[..];
        let (lcs, _) = LogColorSpace::parse(&mut reader).unwrap();
        assert_eq!(lcs.filename.as_deref(), Some(""));
    }

    #[test]
    fn parse_rejects_invalid_signature() {
        let mut data = build_payload(FIXED_PREFIX, false);
        data[..4].copy_from_slice(&0xDEAD_BEEF_u32.to_le_bytes()); // bad sig
        let mut reader = &data[..];
        let err = LogColorSpace::parse(&mut reader).unwrap_err();
        assert!(matches!(err, crate::parser::ParseError::MismatchedField {
            field: "signature",
            ..
        },));
    }
}
