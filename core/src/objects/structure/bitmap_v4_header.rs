/// The BitmapV4Header Object contains information about the dimensions and
/// color format of a device-independent bitmap (DIB). It is an extension of the
/// BitmapInfoHeader Object. (Windows NT 3.1, Windows NT 3.5, and Windows NT
/// 3.51: This structure is not supported.)
#[derive(Clone, Debug)]
pub struct BitmapV4Header {
    /// BitmapInfoHeader (40 bytes): A BitmapInfoHeader object, which defines
    /// properties of the DIB.
    pub bitmap_info_header: crate::BitmapInfoHeader,
    /// RedMask (4 bytes): A 32-bit unsigned integer that defines the color
    /// mask that specifies the red component of each pixel. If the Compression
    /// value in the BitmapInfoHeader object is not BI_BITFIELDS, this value
    /// MUST be ignored.
    pub red_mask: u32,
    /// GreenMask (4 bytes): A 32-bit unsigned integer that defines the color
    /// mask that specifies the green component of each pixel. If the
    /// Compression value in the BitmapInfoHeader object is not BI_BITFIELDS,
    /// this value MUST be ignored.
    pub green_mask: u32,
    /// BlueMask (4 bytes): A 32-bit unsigned integer that defines the color
    /// mask that specifies the blue component of each pixel. If the
    /// Compression value in the BitmapInfoHeader object is not BI_BITFIELDS,
    /// this value MUST be ignored.
    pub blue_mask: u32,
    /// AlphaMask (4 bytes): A 32-bit unsigned integer that defines the color
    /// mask that specifies the alpha component of each pixel.
    pub alpha_mask: u32,
    /// ColorSpaceType (4 bytes): A 32-bit unsigned integer that defines the
    /// color space of the DeviceIndependentBitmap Object. If this value is
    /// LCS_CALIBRATED_RGB from the LogicalColorSpace Enumeration, the color
    /// values in the DIB are calibrated RGB values, and the endpoints and
    /// gamma values in this structure SHOULD be used to translate the color
    /// values before they are passed to the device.
    ///
    /// See the LogColorSpace and LogColorSpace ObjectW objects for details
    /// concerning a logical color space.
    pub color_space_type: crate::LogicalColorSpace,
    /// Endpoints (36 bytes): A CIEXYZTriple Object that defines the CIE
    /// chromaticity x, y, and z coordinates of the three colors that
    /// correspond to the red, green, and blue endpoints for the logical color
    /// space associated with the DIB. If the ColorSpaceType field does not
    /// specify LCS_CALIBRATED_RGB, this field MUST be ignored.
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
    ///
    /// The gamma value format is an unsigned "8.8" fixed-point integer that is
    /// then left-shifted by 8 bits. "8.8" means "8 integer bits followed by 8
    /// fraction bits": nnnnnnnnffffffff. Taking the shift into account, the
    /// required format of the 32-bit DWORD is:
    /// 00000000nnnnnnnnffffffff00000000.
    pub gamma_blue: f32,
}

impl BitmapV4Header {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (
            (bitmap_info_header, bitmap_info_header_bytes),
            (red_mask, red_mask_bytes),
            (green_mask, green_mask_bytes),
            (blue_mask, blue_mask_bytes),
            (alpha_mask, alpha_mask_bytes),
            (color_space_type, color_space_type_bytes),
            (endpoints, endpoints_bytes),
            (gamma_red, gamma_red_bytes),
            (gamma_green, gamma_green_bytes),
            (gamma_blue, gamma_blue_bytes),
        ) = (
            crate::BitmapInfoHeader::parse(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::read_u32_from_le_bytes(buf)?,
            crate::LogicalColorSpace::parse(buf)?,
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
        let consumed_bytes = bitmap_info_header_bytes
            + red_mask_bytes
            + green_mask_bytes
            + blue_mask_bytes
            + alpha_mask_bytes
            + color_space_type_bytes
            + endpoints_bytes
            + gamma_red_bytes
            + gamma_green_bytes
            + gamma_blue_bytes;

        Ok((
            Self {
                bitmap_info_header,
                red_mask,
                green_mask,
                blue_mask,
                alpha_mask,
                color_space_type,
                endpoints,
                gamma_red,
                gamma_green,
                gamma_blue,
            },
            consumed_bytes,
        ))
    }
}
