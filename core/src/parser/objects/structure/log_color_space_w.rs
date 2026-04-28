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

        if signature != 0x50534F43 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The signature field must be 0x50534F43".to_owned(),
            });
        }

        if version != 0x00000400 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The version field must be 0x00000400".to_owned(),
            });
        }

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
