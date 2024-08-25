/// The DeviceIndependentBitmap (DIB) Object defines an image in
/// device-independent bitmap (DIB) format.
#[derive(Clone, Debug)]
pub struct DeviceIndependentBitmap {
    /// DIBHeaderInfo (variable): Either a BitmapCoreHeader Object or a
    /// BitmapInfoHeader Object that specifies information about the image.
    ///
    /// The first 32 bits of this field is the HeaderSize value. If it is
    /// 0x0000000C, then this is a BitmapCoreHeader; otherwise, this is a
    /// BitmapInfoHeader.
    pub dib_header_info: DIBHeaderInfo,
    /// Colors (variable): An optional array of either RGBQuad Objects or
    /// 16-bit unsigned integers that define a color table.
    ///
    /// The size and contents of this field SHOULD be determined from the
    /// metafile record or object that contains this DeviceIndependentBitmap
    /// Object and from information in the DIBHeaderInfo field. See ColorUsage
    /// Enumeration and BitCount Enumeration for additional details.
    pub colors: Colors,
    /// BitmapBuffer (variable): A buffer containing the image, which is not
    /// required to be contiguous with the DIB header, unless this is a packed
    /// bitmap.
    pub bitmap_buffer: BitmapBuffer,
}

impl DeviceIndependentBitmap {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub(crate) fn parse_with_color_usage<R: std::io::Read>(
        buf: &mut R,
        color_usage: crate::ColorUsage,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (dib_header_info, mut consumed_bytes) = DIBHeaderInfo::parse(buf)?;
        let bit_count = dib_header_info.bit_count();
        let colors_length = match bit_count {
            crate::BitCount::BI_BITCOUNT_0 => 0,
            crate::BitCount::BI_BITCOUNT_1 => 2,
            crate::BitCount::BI_BITCOUNT_2 => 2_usize.pow(4),
            crate::BitCount::BI_BITCOUNT_3 => 2_usize.pow(8),
            crate::BitCount::BI_BITCOUNT_4 => match &dib_header_info {
                DIBHeaderInfo::Core(_) => 2_usize.pow(16),
                DIBHeaderInfo::Info(v) => match v.compression {
                    crate::Compression::BI_RGB => 0,
                    _ => 2_usize.pow(16),
                },
            },
            crate::BitCount::BI_BITCOUNT_5 => 0,
            crate::BitCount::BI_BITCOUNT_6 => match &dib_header_info {
                DIBHeaderInfo::Core(_) => 2_usize.pow(24),
                DIBHeaderInfo::Info(v) => match v.compression {
                    crate::Compression::BI_RGB => 0,
                    _ => 2_usize.pow(24),
                },
            },
        };
        let colors = match color_usage {
            crate::ColorUsage::DIB_RGB_COLORS => {
                let mut rgb = vec![];

                for _ in 0..colors_length {
                    let (v, c) = crate::RGBQuad::parse(buf)?;

                    consumed_bytes += c;
                    rgb.push(v);
                }

                Colors::RGBQuad(rgb)
            }
            crate::ColorUsage::DIB_PAL_COLORS => {
                let mut table = vec![];

                for _ in 0..colors_length {
                    let (v, c) = crate::read_u16_from_le_bytes(buf)?;

                    consumed_bytes += c;
                    table.push(v);
                }

                Colors::ColorTable(table)
            }
            crate::ColorUsage::DIB_PAL_INDICES => Colors::Null,
        };

        //  TODO: Not written in [MS-WMF] how to parse this field.
        let undefined_space = vec![];
        let (a_data, c) = crate::read_variable(buf, dib_header_info.size())?;
        consumed_bytes += c;

        Ok((
            Self {
                dib_header_info,
                colors,
                bitmap_buffer: BitmapBuffer { undefined_space, a_data },
            },
            consumed_bytes,
        ))
    }
}

#[derive(Clone, Debug)]
pub enum Colors {
    ColorTable(Vec<u16>),
    RGBQuad(Vec<crate::RGBQuad>),
    Null,
}

#[derive(Clone, Debug)]
pub enum DIBHeaderInfo {
    Core(crate::BitmapCoreHeader),
    Info(crate::BitmapInfoHeader),
}

impl DIBHeaderInfo {
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (header_size, mut consumed_bytes) =
            crate::read_u32_from_le_bytes(buf)?;

        if header_size == 0x0000000C {
            let (header, c) = crate::BitmapCoreHeader::parse_with_header_size(
                buf,
                header_size,
            )?;
            consumed_bytes += c;

            Ok((Self::Core(header), consumed_bytes))
        } else {
            let (header, c) = crate::BitmapInfoHeader::parse_with_header_size(
                buf,
                header_size,
            )?;
            consumed_bytes += c;

            Ok((Self::Info(header), consumed_bytes))
        }
    }
}

impl DIBHeaderInfo {
    fn bit_count(&self) -> crate::BitCount {
        match &self {
            DIBHeaderInfo::Core(v) => v.bit_count,
            DIBHeaderInfo::Info(v) => v.bit_count,
        }
    }

    fn size(&self) -> usize {
        let size = match self {
            DIBHeaderInfo::Core(v) => u32::from(
                (((v.width * v.planes * (v.bit_count as u16) + 31) & !31) / 8)
                    * v.height,
            ),
            DIBHeaderInfo::Info(v) => match v.compression {
                crate::Compression::BI_RGB
                | crate::Compression::BI_BITFIELDS
                | crate::Compression::BI_CMYK => u32::from(
                    ((((v.width as u32)
                        * u32::from(v.planes)
                        * (v.bit_count as u32)
                        + 31)
                        & !31)
                        / 8)
                        * v.height.abs() as u32,
                ),
                _ => v.image_size,
            },
        };

        size as usize
    }
}

#[derive(Clone)]
pub struct BitmapBuffer {
    /// UndefinedSpace (variable): An optional field that MUST be ignored. If
    /// this DIB is a packed bitmap, this field MUST NOT be present.
    pub undefined_space: Vec<u8>,
    /// aData (variable): An array of bytes that define the image.
    ///
    /// The size and format of this data is determined by information in the
    /// DIBHeaderInfo field. If it is a BitmapCoreHeader, the size in bytes
    /// MUST be calculated as follows:
    ///
    /// ```
    /// (((Width * Planes * BitCount + 31) & ~31) / 8) * abs(Height)
    /// ```
    ///
    /// This formula SHOULD also be used to calculate the size of aData when
    /// DIBHeaderInfo is a BitmapInfoHeader Object, using values from that
    /// object, but only if its Compression value is BI_RGB, BI_BITFIELDS, or
    /// BI_CMYK.
    ///
    /// Otherwise, the size of aData MUST be the BitmapInfoHeader Object value
    /// ImageSize.
    pub a_data: Vec<u8>,
}

impl std::fmt::Debug for BitmapBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitmapBuffer")
            .field(
                "undefined_space",
                &format!("[u8; {}]", self.undefined_space.len()),
            )
            .field("a_data", &format!("[u8; {}]", self.a_data.len()))
            .finish()
    }
}
