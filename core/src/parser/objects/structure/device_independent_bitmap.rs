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
    pub dib_header_info: crate::parser::BitmapInfoHeader,
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
        err(level = tracing::Level::ERROR, Display),
    )]
    pub(crate) fn parse_with_color_usage<R: std::io::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (dib_header_info, mut consumed_bytes) =
            crate::parser::BitmapInfoHeader::parse(buf)?;
        let (colors, c) = Colors::parse(buf, color_usage, &dib_header_info)?;
        consumed_bytes += c;

        //  TODO: Not written in [MS-WMF] how to parse this field.
        let undefined_space = vec![];
        let (a_data, c) =
            crate::parser::read_variable(buf, dib_header_info.size())?;
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
    PaletteIndices(Vec<u16>),
    RGBQuad(Vec<crate::parser::RGBQuad>),
    RGBColorMask(Vec<(u32, u32, u32)>),
    Null,
}

impl Colors {
    fn parse<R: std::io::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
        dib_header_info: &crate::parser::BitmapInfoHeader,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        match dib_header_info.bit_count() {
            crate::parser::BitCount::BI_BITCOUNT_0
            | crate::parser::BitCount::BI_BITCOUNT_5 => Ok((Colors::Null, 0)),
            crate::parser::BitCount::BI_BITCOUNT_1 => {
                Self::parse_from_color_usage(buf, color_usage, 2)
            }
            crate::parser::BitCount::BI_BITCOUNT_2 => {
                Self::parse_from_color_usage(buf, color_usage, 2_usize.pow(4))
            }
            crate::parser::BitCount::BI_BITCOUNT_3 => {
                Self::parse_from_color_usage(buf, color_usage, 2_usize.pow(8))
            }
            crate::parser::BitCount::BI_BITCOUNT_4
            | crate::parser::BitCount::BI_BITCOUNT_6 => {
                match &dib_header_info {
                    crate::parser::BitmapInfoHeader::Core { .. } => {
                        Ok((Colors::Null, 0))
                    }
                    crate::parser::BitmapInfoHeader::Info {
                        compression,
                        color_used,
                        ..
                    }
                    | crate::parser::BitmapInfoHeader::V4 {
                        compression,
                        color_used,
                        ..
                    }
                    | crate::parser::BitmapInfoHeader::V5 {
                        compression,
                        color_used,
                        ..
                    } => match compression {
                        crate::parser::Compression::BI_RGB => {
                            Ok((Colors::Null, 0))
                        }
                        crate::parser::Compression::BI_BITFIELDS => {
                            let mut consumed_bytes = 0;
                            let mut table = vec![];

                            for _ in 0..*color_used {
                                let ((r, r_bytes), (g, g_bytes), (b, b_bytes)) = (
                                    crate::parser::read_u32_from_le_bytes(buf)?,
                                    crate::parser::read_u32_from_le_bytes(buf)?,
                                    crate::parser::read_u32_from_le_bytes(buf)?,
                                );

                                consumed_bytes += r_bytes + g_bytes + b_bytes;
                                table.push((r, g, b));
                            }

                            Ok((Colors::RGBColorMask(table), consumed_bytes))
                        }
                        _ => Ok((Colors::Null, 0)),
                    },
                }
            }
        }
    }

    fn parse_from_color_usage<R: std::io::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
        colors_length: usize,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let mut consumed_bytes: usize = 0;

        match color_usage {
            crate::parser::ColorUsage::DIB_RGB_COLORS => {
                let mut table = vec![];

                for _ in 0..colors_length {
                    let (v, c) = crate::parser::RGBQuad::parse(buf)?;

                    consumed_bytes += c;
                    table.push(v);
                }

                Ok((Colors::RGBQuad(table), consumed_bytes))
            }
            crate::parser::ColorUsage::DIB_PAL_COLORS => {
                let mut table = vec![];

                for _ in 0..colors_length {
                    let (v, c) = crate::parser::read_u16_from_le_bytes(buf)?;

                    consumed_bytes += c;
                    table.push(v);
                }

                Ok((Colors::PaletteIndices(table), consumed_bytes))
            }
            crate::parser::ColorUsage::DIB_PAL_INDICES => {
                Ok((Colors::Null, consumed_bytes))
            }
        }
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
