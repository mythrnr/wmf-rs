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
    pub color_space_type: crate::LogicalColorSpace,
    /// Intent (4 bytes): A 32-bit signed integer that defines the gamut
    /// mapping intent. It MUST be defined in the GamutMappingIntent
    /// Enumeration.
    pub intent: crate::GamutMappingIntent,
    /// Endpoints (36 bytes): A CIEXYZTriple Object that defines the CIE
    /// chromaticity x, y, and z coordinates of the three colors that
    /// correspond to the RGB endpoints for the logical color space associated
    /// with the bitmap. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub endpoints: crate::CIEXYZTriple,
    /// GammaRed (4 bytes): A 32-bit fixed point value that defines the toned
    /// response curve for red. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub gamma_red: f32,
    /// GammaGreen (4 bytes): A 32-bit fixed point value that defines the toned
    /// response curve for green. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub gamma_green: f32,
    /// GammaBlue (4 bytes): A 32-bit fixed point value that defines the toned
    /// response curve for blue. If the ColorSpaceType field does not specify
    /// LCS_CALIBRATED_RGB, this field MUST be ignored.
    pub gamma_blue: f32,
    /// Filename (260 bytes): An optional, ASCII charactger string that
    /// specifies the name of a file that contains a color profile. If a file
    /// name is specified, and the ColorSpaceType field is set to
    /// LCS_CALIBRATED_RGB, the other fields of this structure SHOULD be
    /// ignored.
    pub filename: Option<String>,
}

impl LogColorSpace {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (
            (signature, signature_bytes),
            (version, version_bytes),
            (size, size_bytes),
            (color_space_type, color_space_type_bytes),
            (intent, intent_bytes),
            (endpoints, endpoints_bytes),
            (gamma_red, gamma_red_bytes),
            (gamma_green, gamma_green_bytes),
            (gamma_blue, gamma_blue_bytes),
        ) = (
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::LogicalColorSpace::parse(buf)?,
            crate::GamutMappingIntent::parse(buf)?,
            crate::CIEXYZTriple::parse(buf)?,
            {
                let (v, c) = crate::read::<R, 4>(buf)?;
                (crate::objects::structure::f32_from_fixed_point_q_8_8(v), c)
            },
            {
                let (v, c) = crate::read::<R, 4>(buf)?;
                (crate::objects::structure::f32_from_fixed_point_q_8_8(v), c)
            },
            {
                let (v, c) = crate::read::<R, 4>(buf)?;
                (crate::objects::structure::f32_from_fixed_point_q_8_8(v), c)
            },
        );
        let mut consumed_bytes = signature_bytes
            + version_bytes
            + size_bytes
            + color_space_type_bytes
            + intent_bytes
            + endpoints_bytes
            + gamma_red_bytes
            + gamma_green_bytes
            + gamma_blue_bytes;

        let filename = if size as usize - consumed_bytes >= 260 {
            let (bytes, filename_bytes) = crate::read_variable(buf, 260)?;
            consumed_bytes += filename_bytes;

            Some(String::from_utf8_lossy(&bytes).to_string())
        } else {
            None
        };

        if signature != 0x50534F43 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: "The signature field must be 0x50534F43".to_owned(),
            });
        }

        if version != 0x00000400 {
            return Err(crate::ParseError::UnexpectedPattern {
                cause: "The version field must be 0x00000400".to_owned(),
            });
        }

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
