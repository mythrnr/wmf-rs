mod core;
mod info;
mod v4;
mod v5;

#[derive(Clone, Debug)]
pub enum BitmapInfoHeader {
    /// The BitmapCoreHeader Object contains information about the dimensions
    /// and color format of a device-independent bitmap (DIB). (Although
    /// Windows processes BitmapCoreHeader objects in DIBs, it does not
    /// write them to WMF metafiles)
    ///
    /// A DIB is specified by a DeviceIndependentBitmap Object.
    Core {
        /// HeaderSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size of this object, in bytes.
        header_size: u32,
        /// Width (2 bytes): A 16-bit unsigned integer that defines the width
        /// of the DIB, in pixels.
        width: u16,
        /// Height (2 bytes): A 16-bit unsigned integer that defines the height
        /// of the DIB, in pixels.
        height: u16,
        /// Planes (2 bytes): A 16-bit unsigned integer that defines the number
        /// of planes for the target device. This value MUST be 0x0001.
        planes: u16,
        /// BitCount (2 bytes): A 16-bit unsigned integer that defines the
        /// format of each pixel, and the maximum number of colors in the DIB.
        /// This value MUST be in the BitCount Enumeration.
        bit_count: crate::parser::BitCount,
    },
    /// The BitmapInfoHeader Object contains information about the dimensions
    /// and color format of a device-independent bitmap (DIB).
    Info {
        /// HeaderSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size of this object, in bytes.
        header_size: u32,
        /// Width (4 bytes): A 32-bit signed integer that defines the width of
        /// the DIB, in pixels. This value MUST be positive.
        ///
        /// This field SHOULD specify the width of the decompressed image file,
        /// if the Compression value specifies JPEG or PNG format. (Windows NT
        /// 3.1, Windows NT 3.5, Windows NT 3.51, Windows 95, and Windows NT
        /// 4.0: Neither JPEG nor PNG format is supported.)
        width: i32,
        /// Height (4 bytes): A 32-bit signed integer that defines the height
        /// of the DIB, in pixels. This value MUST NOT be zero.
        ///
        /// | Value | Meaning |
        /// |-|-|
        /// | 0x00000000 < value | If this value is positive, the DIB is a bottom-up bitmap, and its origin is the lower-left corner. This field SHOULD specify the height of the decompressed image file, if the Compression value specifies JPEG or PNG format. |
        /// | value < 0x00000000 | If this value is negative, the DIB is a top-down bitmap, and its origin is the upper-left corner. Top-down bitmaps do not support compression. |
        height: i32,
        /// Planes (2 bytes): A 16-bit unsigned integer that defines the number
        /// of planes for the target device. This value MUST be 0x0001.
        planes: u16,
        /// BitCount (2 bytes): A 16-bit unsigned integer that defines the
        /// number of bits that define each pixel and the maximum number of
        /// colors in the DIB. This value MUST be in the BitCount Enumeration.
        bit_count: crate::parser::BitCount,
        /// Compression (4 bytes): A 32-bit unsigned integer that defines the
        /// compression mode of the DIB. This value MUST be in the Compression
        /// Enumeration.
        ///
        /// This value MUST NOT specify a compressed format if the DIB is a
        /// top-down bitmap, as indicated by the Height value.
        compression: crate::parser::Compression,
        /// ImageSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size, in bytes, of the image.
        ///
        /// If the Compression value is BI_RGB, this value SHOULD be zero and
        /// MUST be ignored. (Windows implementations might write a nonzero
        /// value to this field, but it is ignored when the metafile is
        /// parsed.)
        ///
        /// If the Compression value is BI_JPEG or BI_PNG, this value MUST
        /// specify the size of the JPEG or PNG image buffer, respectively.
        image_size: u32,
        /// XPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
        /// horizontal resolution, in pixels-per-meter, of the target device
        /// for the DIB.
        x_pels_per_meter: i32,
        /// YPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
        /// vertical resolution, in pixels-per-meter, of the target device for
        /// the DIB.
        y_pels_per_meter: i32,
        /// ColorUsed (4 bytes): A 32-bit unsigned integer that specifies the
        /// number of indexes in the color table used by the DIB, as follows:
        ///
        /// - If this value is zero, the DIB uses the maximum number of colors
        ///   that correspond to the BitCount value.
        /// - If this value is nonzero and the BitCount value is less than 16,
        ///   this value specifies the number of colors used by the DIB.
        /// - If this value is nonzero and the BitCount value is 16 or greater,
        ///   this value specifies the size of the color table used to optimize
        ///   performance of the system palette.
        ///
        /// Note If this value is nonzero and greater than the maximum possible
        /// size of the color table based on the BitCount value, the maximum
        /// color table size SHOULD be assumed.
        color_used: u32,
        /// ColorImportant (4 bytes): A 32-bit unsigned integer that defines
        /// the number of color indexes that are required for displaying the
        /// DIB. If this value is zero, all color indexes are required.
        ///
        /// A DIB is specified by a DeviceIndependentBitmap Object.
        ///
        /// When the array of pixels in the DIB immediately follows the
        /// BitmapInfoHeader, the DIB is a packed bitmap. In a packed bitmap,
        /// the ColorUsed value MUST be either 0x00000000 or the actual size of
        /// the color table.
        color_important: u32,
    },
    /// The BitmapV4Header Object contains information about the dimensions and
    /// color format of a device-independent bitmap (DIB). It is an extension
    /// of the BitmapInfoHeader Object. (Windows NT 3.1, Windows NT 3.5, and
    /// Windows NT 3.51: This structure is not supported.)
    V4 {
        /// HeaderSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size of this object, in bytes.
        header_size: u32,
        /// Width (4 bytes): A 32-bit signed integer that defines the width of
        /// the DIB, in pixels. This value MUST be positive.
        ///
        /// This field SHOULD specify the width of the decompressed image file,
        /// if the Compression value specifies JPEG or PNG format. (Windows NT
        /// 3.1, Windows NT 3.5, Windows NT 3.51, Windows 95, and Windows NT
        /// 4.0: Neither JPEG nor PNG format is supported.)
        width: i32,
        /// Height (4 bytes): A 32-bit signed integer that defines the height
        /// of the DIB, in pixels. This value MUST NOT be zero.
        ///
        /// | Value | Meaning |
        /// |-|-|
        /// | 0x00000000 < value | If this value is positive, the DIB is a bottom-up bitmap, and its origin is the lower-left corner. This field SHOULD specify the height of the decompressed image file, if the Compression value specifies JPEG or PNG format. |
        /// | value < 0x00000000 | If this value is negative, the DIB is a top-down bitmap, and its origin is the upper-left corner. Top-down bitmaps do not support compression. |
        height: i32,
        /// Planes (2 bytes): A 16-bit unsigned integer that defines the number
        /// of planes for the target device. This value MUST be 0x0001.
        planes: u16,
        /// BitCount (2 bytes): A 16-bit unsigned integer that defines the
        /// number of bits that define each pixel and the maximum number of
        /// colors in the DIB. This value MUST be in the BitCount Enumeration.
        bit_count: crate::parser::BitCount,
        /// Compression (4 bytes): A 32-bit unsigned integer that defines the
        /// compression mode of the DIB. This value MUST be in the Compression
        /// Enumeration.
        ///
        /// This value MUST NOT specify a compressed format if the DIB is a
        /// top-down bitmap, as indicated by the Height value.
        compression: crate::parser::Compression,
        /// ImageSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size, in bytes, of the image.
        ///
        /// If the Compression value is BI_RGB, this value SHOULD be zero and
        /// MUST be ignored. (Windows implementations might write a nonzero
        /// value to this field, but it is ignored when the metafile is
        /// parsed.)
        ///
        /// If the Compression value is BI_JPEG or BI_PNG, this value MUST
        /// specify the size of the JPEG or PNG image buffer, respectively.
        image_size: u32,
        /// XPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
        /// horizontal resolution, in pixels-per-meter, of the target device
        /// for the DIB.
        x_pels_per_meter: i32,
        /// YPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
        /// vertical resolution, in pixels-per-meter, of the target device for
        /// the DIB.
        y_pels_per_meter: i32,
        /// ColorUsed (4 bytes): A 32-bit unsigned integer that specifies the
        /// number of indexes in the color table used by the DIB, as follows:
        ///
        /// - If this value is zero, the DIB uses the maximum number of colors
        ///   that correspond to the BitCount value.
        /// - If this value is nonzero and the BitCount value is less than 16,
        ///   this value specifies the number of colors used by the DIB.
        /// - If this value is nonzero and the BitCount value is 16 or greater,
        ///   this value specifies the size of the color table used to optimize
        ///   performance of the system palette.
        ///
        /// Note If this value is nonzero and greater than the maximum possible
        /// size of the color table based on the BitCount value, the maximum
        /// color table size SHOULD be assumed.
        color_used: u32,
        /// ColorImportant (4 bytes): A 32-bit unsigned integer that defines
        /// the number of color indexes that are required for displaying the
        /// DIB. If this value is zero, all color indexes are required.
        ///
        /// A DIB is specified by a DeviceIndependentBitmap Object.
        ///
        /// When the array of pixels in the DIB immediately follows the
        /// BitmapInfoHeader, the DIB is a packed bitmap. In a packed bitmap,
        /// the ColorUsed value MUST be either 0x00000000 or the actual size of
        /// the color table.
        color_important: u32,
        /// RedMask (4 bytes): A 32-bit unsigned integer that defines the color
        /// mask that specifies the red component of each pixel. If the
        /// Compression value in the BitmapInfoHeader object is not
        /// BI_BITFIELDS, this value MUST be ignored.
        red_mask: u32,
        /// GreenMask (4 bytes): A 32-bit unsigned integer that defines the
        /// color mask that specifies the green component of each pixel. If the
        /// Compression value in the BitmapInfoHeader object is not
        /// BI_BITFIELDS, this value MUST be ignored.
        green_mask: u32,
        /// BlueMask (4 bytes): A 32-bit unsigned integer that defines the
        /// color mask that specifies the blue component of each pixel. If the
        /// Compression value in the BitmapInfoHeader object is not
        /// BI_BITFIELDS, this value MUST be ignored.
        blue_mask: u32,
        /// AlphaMask (4 bytes): A 32-bit unsigned integer that defines the
        /// color mask that specifies the alpha component of each pixel.
        alpha_mask: u32,
        /// ColorSpaceType (4 bytes): A 32-bit unsigned integer that defines
        /// the color space of the DeviceIndependentBitmap Object. If this
        /// value is LCS_CALIBRATED_RGB from the LogicalColorSpace Enumeration,
        /// the color values in the DIB are calibrated RGB values, and the
        /// endpoints and gamma values in this structure SHOULD be used to
        /// translate the color values before they are passed to the device.
        ///
        /// See the LogColorSpace and LogColorSpace ObjectW objects for details
        /// concerning a logical color space.
        color_space_type: crate::parser::LogicalColorSpace,
        /// Endpoints (36 bytes): A CIEXYZTriple Object that defines the CIE
        /// chromaticity x, y, and z coordinates of the three colors that
        /// correspond to the red, green, and blue endpoints for the logical
        /// color space associated with the DIB. If the ColorSpaceType field
        /// does not specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        endpoints: crate::parser::CIEXYZTriple,
        /// GammaRed (4 bytes): A 32-bit fixed point value that defines the
        /// toned response curve for red. If the ColorSpaceType field does not
        /// specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        gamma_red: u32,
        /// GammaGreen (4 bytes): A 32-bit fixed point value that defines the
        /// toned response curve for green. If the ColorSpaceType field does
        /// not specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        gamma_green: u32,
        /// GammaBlue (4 bytes): A 32-bit fixed point value that defines the
        /// toned response curve for blue. If the ColorSpaceType field does not
        /// specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        ///
        /// The gamma value format is an unsigned "8.8" fixed-point integer
        /// that is then left-shifted by 8 bits. "8.8" means "8 integer bits
        /// followed by 8 fraction bits": nnnnnnnnffffffff. Taking the shift
        /// into account, the required format of the 32-bit DWORD is:
        /// 00000000nnnnnnnnffffffff00000000.
        gamma_blue: u32,
    },
    /// The BitmapV5Header Object contains information about the dimensions and
    /// color format of a device-independent bitmap (DIB). It is an extension
    /// of the BitmapV4Header Object. (Windows NT 3.1, Windows NT 3.5, Windows
    /// NT 3.51, Windows 95, and Windows NT 4.0: This structure is not
    /// supported.)
    V5 {
        /// HeaderSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size of this object, in bytes.
        header_size: u32,
        /// Width (4 bytes): A 32-bit signed integer that defines the width of
        /// the DIB, in pixels. This value MUST be positive.
        ///
        /// This field SHOULD specify the width of the decompressed image file,
        /// if the Compression value specifies JPEG or PNG format. (Windows NT
        /// 3.1, Windows NT 3.5, Windows NT 3.51, Windows 95, and Windows NT
        /// 4.0: Neither JPEG nor PNG format is supported.)
        width: i32,
        /// Height (4 bytes): A 32-bit signed integer that defines the height
        /// of the DIB, in pixels. This value MUST NOT be zero.
        ///
        /// | Value | Meaning |
        /// |-|-|
        /// | 0x00000000 < value | If this value is positive, the DIB is a bottom-up bitmap, and its origin is the lower-left corner. This field SHOULD specify the height of the decompressed image file, if the Compression value specifies JPEG or PNG format. |
        /// | value < 0x00000000 | If this value is negative, the DIB is a top-down bitmap, and its origin is the upper-left corner. Top-down bitmaps do not support compression. |
        height: i32,
        /// Planes (2 bytes): A 16-bit unsigned integer that defines the number
        /// of planes for the target device. This value MUST be 0x0001.
        planes: u16,
        /// BitCount (2 bytes): A 16-bit unsigned integer that defines the
        /// number of bits that define each pixel and the maximum number of
        /// colors in the DIB. This value MUST be in the BitCount Enumeration.
        bit_count: crate::parser::BitCount,
        /// Compression (4 bytes): A 32-bit unsigned integer that defines the
        /// compression mode of the DIB. This value MUST be in the Compression
        /// Enumeration.
        ///
        /// This value MUST NOT specify a compressed format if the DIB is a
        /// top-down bitmap, as indicated by the Height value.
        compression: crate::parser::Compression,
        /// ImageSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size, in bytes, of the image.
        ///
        /// If the Compression value is BI_RGB, this value SHOULD be zero and
        /// MUST be ignored. (Windows implementations might write a nonzero
        /// value to this field, but it is ignored when the metafile is
        /// parsed.)
        ///
        /// If the Compression value is BI_JPEG or BI_PNG, this value MUST
        /// specify the size of the JPEG or PNG image buffer, respectively.
        image_size: u32,
        /// XPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
        /// horizontal resolution, in pixels-per-meter, of the target device
        /// for the DIB.
        x_pels_per_meter: i32,
        /// YPelsPerMeter (4 bytes): A 32-bit signed integer that defines the
        /// vertical resolution, in pixels-per-meter, of the target device for
        /// the DIB.
        y_pels_per_meter: i32,
        /// ColorUsed (4 bytes): A 32-bit unsigned integer that specifies the
        /// number of indexes in the color table used by the DIB, as follows:
        ///
        /// - If this value is zero, the DIB uses the maximum number of colors
        ///   that correspond to the BitCount value.
        /// - If this value is nonzero and the BitCount value is less than 16,
        ///   this value specifies the number of colors used by the DIB.
        /// - If this value is nonzero and the BitCount value is 16 or greater,
        ///   this value specifies the size of the color table used to optimize
        ///   performance of the system palette.
        ///
        /// Note If this value is nonzero and greater than the maximum possible
        /// size of the color table based on the BitCount value, the maximum
        /// color table size SHOULD be assumed.
        color_used: u32,
        /// ColorImportant (4 bytes): A 32-bit unsigned integer that defines
        /// the number of color indexes that are required for displaying the
        /// DIB. If this value is zero, all color indexes are required.
        ///
        /// A DIB is specified by a DeviceIndependentBitmap Object.
        ///
        /// When the array of pixels in the DIB immediately follows the
        /// BitmapInfoHeader, the DIB is a packed bitmap. In a packed bitmap,
        /// the ColorUsed value MUST be either 0x00000000 or the actual size of
        /// the color table.
        color_important: u32,
        /// RedMask (4 bytes): A 32-bit unsigned integer that defines the color
        /// mask that specifies the red component of each pixel. If the
        /// Compression value in the BitmapInfoHeader object is not
        /// BI_BITFIELDS, this value MUST be ignored.
        red_mask: u32,
        /// GreenMask (4 bytes): A 32-bit unsigned integer that defines the
        /// color mask that specifies the green component of each pixel. If the
        /// Compression value in the BitmapInfoHeader object is not
        /// BI_BITFIELDS, this value MUST be ignored.
        green_mask: u32,
        /// BlueMask (4 bytes): A 32-bit unsigned integer that defines the
        /// color mask that specifies the blue component of each pixel. If the
        /// Compression value in the BitmapInfoHeader object is not
        /// BI_BITFIELDS, this value MUST be ignored.
        blue_mask: u32,
        /// AlphaMask (4 bytes): A 32-bit unsigned integer that defines the
        /// color mask that specifies the alpha component of each pixel.
        alpha_mask: u32,
        /// ColorSpaceType (4 bytes): A 32-bit unsigned integer that defines
        /// the color space of the DeviceIndependentBitmap Object. If this
        /// value is LCS_CALIBRATED_RGB from the LogicalColorSpace Enumeration,
        /// the color values in the DIB are calibrated RGB values, and the
        /// endpoints and gamma values in this structure SHOULD be used to
        /// translate the color values before they are passed to the device.
        ///
        /// See the LogColorSpace and LogColorSpace ObjectW objects for details
        /// concerning a logical color space.
        color_space_type: crate::parser::LogicalColorSpace,
        /// Endpoints (36 bytes): A CIEXYZTriple Object that defines the CIE
        /// chromaticity x, y, and z coordinates of the three colors that
        /// correspond to the red, green, and blue endpoints for the logical
        /// color space associated with the DIB. If the ColorSpaceType field
        /// does not specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        endpoints: crate::parser::CIEXYZTriple,
        /// GammaRed (4 bytes): A 32-bit fixed point value that defines the
        /// toned response curve for red. If the ColorSpaceType field does not
        /// specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        gamma_red: u32,
        /// GammaGreen (4 bytes): A 32-bit fixed point value that defines the
        /// toned response curve for green. If the ColorSpaceType field does
        /// not specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        gamma_green: u32,
        /// GammaBlue (4 bytes): A 32-bit fixed point value that defines the
        /// toned response curve for blue. If the ColorSpaceType field does not
        /// specify LCS_CALIBRATED_RGB, this field MUST be ignored.
        ///
        /// The gamma value format is an unsigned "8.8" fixed-point integer
        /// that is then left-shifted by 8 bits. "8.8" means "8 integer bits
        /// followed by 8 fraction bits": nnnnnnnnffffffff. Taking the shift
        /// into account, the required format of the 32-bit DWORD is:
        /// 00000000nnnnnnnnffffffff00000000.
        gamma_blue: u32,
        /// Intent (4 bytes): A 32-bit unsigned integer that defines the
        /// rendering intent for the DIB. This MUST be a value defined in the
        /// GamutMappingIntent Enumeration.
        intent: crate::parser::GamutMappingIntent,
        /// ProfileData (4 bytes): A 32-bit unsigned integer that defines the
        /// offset, in bytes, from the beginning of this structure to the start
        /// of the color profile data.
        ///
        /// If the color profile is embedded in the DIB, ProfileData is the
        /// offset to the actual color profile; if the color profile is linked,
        /// ProfileData is the offset to the null-terminated file name of the
        /// color profile. This MUST NOT be a Unicode string, but MUST be
        /// composed exclusively of characters from the Windows character set
        /// (code page 1252).
        ///
        /// If the ColorSpaceType field in the BitmapV4Header does not specify
        /// LCS_PROFILE_LINKED or LCS_PROFILE_EMBEDDED, the color profile data
        /// SHOULD be ignored.
        profile_data: u32,
        /// ProfileSize (4 bytes): A 32-bit unsigned integer that defines the
        /// size, in bytes, of embedded color profile data.
        profile_size: u32,
        /// Reserved (4 bytes): A 32-bit unsigned integer that is undefined and
        /// SHOULD be ignored.
        reserved: u32,
    },
}

impl BitmapInfoHeader {
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (header_size, mut consumed_bytes) =
            crate::parser::read_u32_from_le_bytes(buf)?;

        match header_size {
            0x0000000C => {
                let (header, c) = Self::parse_as_core(buf, header_size)?;
                consumed_bytes += c;

                Ok((header, consumed_bytes))
            }
            13..=40 => {
                let (header, c) = Self::parse_as_info(buf, header_size)?;
                consumed_bytes += c;

                Ok((header, consumed_bytes))
            }
            41..=108 => {
                let (header, c) = Self::parse_as_v4(buf, header_size)?;
                consumed_bytes += c;

                Ok((header, consumed_bytes))
            }
            109..=124 => {
                let (header, c) = Self::parse_as_v5(buf, header_size)?;
                consumed_bytes += c;

                Ok((header, consumed_bytes))
            }
            _ => {
                return Err(crate::parser::ParseError::UnexpectedPattern {
                    cause: format!(
                        "The header_size `{header_size:#10X}` field is not \
                         match as any BitmapInfoHeader format"
                    ),
                })
            }
        }
    }

    pub fn bit_count(&self) -> crate::parser::BitCount {
        match self {
            Self::Core { bit_count, .. } => *bit_count,
            Self::Info { bit_count, .. } => *bit_count,
            Self::V4 { bit_count, .. } => *bit_count,
            Self::V5 { bit_count, .. } => *bit_count,
        }
    }

    pub fn size(&self) -> usize {
        let size = match self {
            Self::Core { width, height, planes, bit_count, .. } => u32::from(
                (((width * planes * (bit_count.clone() as u16) + 31) & !31)
                    / 8)
                    * height,
            ),
            Self::Info {
                width,
                height,
                planes,
                bit_count,
                image_size,
                compression,
                ..
            }
            | Self::V4 {
                width,
                height,
                planes,
                bit_count,
                image_size,
                compression,
                ..
            }
            | Self::V5 {
                width,
                height,
                planes,
                bit_count,
                image_size,
                compression,
                ..
            } => match compression {
                crate::parser::Compression::BI_RGB
                | crate::parser::Compression::BI_BITFIELDS
                | crate::parser::Compression::BI_CMYK => u32::from(
                    ((((*width as u32)
                        * u32::from(*planes)
                        * (*bit_count as u32)
                        + 31)
                        & !31)
                        / 8)
                        * height.abs() as u32,
                ),
                _ => *image_size,
            },
        };

        size as usize
    }

    pub fn color_used(&self) -> u32 {
        match self {
            Self::Core { bit_count, .. } => 2u32.pow(*bit_count as u32),
            Self::Info { bit_count, color_used, .. }
            | Self::V4 { bit_count, color_used, .. }
            | Self::V5 { bit_count, color_used, .. } => {
                if *color_used == 0
                    && matches!(
                        bit_count,
                        crate::parser::BitCount::BI_BITCOUNT_1
                            | crate::parser::BitCount::BI_BITCOUNT_2
                            | crate::parser::BitCount::BI_BITCOUNT_3
                    )
                {
                    2u32.pow(*bit_count as u32)
                } else {
                    *color_used
                }
            }
        }
    }
}
