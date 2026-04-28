use crate::imports::*;

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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub(crate) fn parse_with_color_usage<R: crate::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_bytes_field, read_with};

        let mut consumed_bytes: usize = 0;
        let dib_header_info = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::BitmapInfoHeader::parse,
        )?;
        let colors = read_with(buf, &mut consumed_bytes, |b| {
            Colors::parse(b, color_usage, &dib_header_info)
        })?;

        // [MS-WMF] does not specify how to derive the length of the
        // UndefinedSpace region, so it is silently skipped: aData follows
        // the color table directly in this implementation.
        let a_data =
            read_bytes_field(buf, &mut consumed_bytes, dib_header_info.size())?;

        Ok((
            Self {
                dib_header_info,
                colors,
                bitmap_buffer: BitmapBuffer { a_data },
            },
            consumed_bytes,
        ))
    }
}

#[derive(Clone, Debug)]
pub enum Colors {
    PaletteIndices(Vec<u16>),
    RGBQuad(Vec<crate::parser::RGBQuad>),
    RGBTriple(Vec<crate::parser::RGBTriple>),
    Null,
}

impl Colors {
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
        dib_header_info: &crate::parser::BitmapInfoHeader,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        if matches!(color_usage, crate::parser::ColorUsage::DIB_PAL_INDICES)
            || matches!(
                dib_header_info.bit_count(),
                crate::parser::BitCount::BI_BITCOUNT_0
            )
        {
            return Ok((Colors::Null, 0));
        }

        if matches!(
            dib_header_info,
            crate::parser::BitmapInfoHeader::Core { .. }
        ) {
            return Self::parse_with_core_header(
                buf,
                color_usage,
                dib_header_info,
            );
        }

        Self::parse_with_info_header(buf, color_usage, dib_header_info)
    }

    fn parse_with_core_header<R: crate::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
        dib_header_info: &crate::parser::BitmapInfoHeader,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        if !matches!(
            dib_header_info,
            crate::parser::BitmapInfoHeader::Core { .. }
        ) {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "expected BitmapInfoHeader::Core variant".to_string(),
            });
        }

        // core header does not have color_used field.
        if matches!(
            dib_header_info.bit_count(),
            crate::parser::BitCount::BI_BITCOUNT_5
        ) {
            return Ok((Colors::Null, 0));
        }

        Self::parse_from_color_usage(
            buf,
            color_usage,
            dib_header_info.color_used() as usize,
            true,
        )
    }

    fn parse_with_info_header<R: crate::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
        dib_header_info: &crate::parser::BitmapInfoHeader,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        if !matches!(
            dib_header_info,
            crate::parser::BitmapInfoHeader::Info { .. }
                | crate::parser::BitmapInfoHeader::V4 { .. }
                | crate::parser::BitmapInfoHeader::V5 { .. }
        ) {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "expected BitmapInfoHeader Info/V4/V5 variant"
                    .to_string(),
            });
        }

        match dib_header_info.bit_count() {
            // `Colors::parse` short-circuits to `Colors::Null` for
            // `BI_BITCOUNT_0`, so reaching here is an internal invariant
            // violation rather than malformed input.
            crate::parser::BitCount::BI_BITCOUNT_0 => unreachable!(
                "BI_BITCOUNT_0 must be handled by Colors::parse before \
                 dispatching to parse_with_info_header"
            ),
            crate::parser::BitCount::BI_BITCOUNT_1
            | crate::parser::BitCount::BI_BITCOUNT_2
            | crate::parser::BitCount::BI_BITCOUNT_3 => {
                Self::parse_from_color_usage(
                    buf,
                    color_usage,
                    dib_header_info.color_used() as usize,
                    false,
                )
            }
            crate::parser::BitCount::BI_BITCOUNT_5 => {
                let bytes = Self::skip_color_usage(
                    buf,
                    color_usage,
                    dib_header_info.color_used() as usize,
                )?;

                Ok((Colors::Null, bytes))
            }

            crate::parser::BitCount::BI_BITCOUNT_4
            | crate::parser::BitCount::BI_BITCOUNT_6 => {
                match &dib_header_info {
                    // `Core` was rejected at the top of this function;
                    // arriving here means the variant guard above was broken.
                    crate::parser::BitmapInfoHeader::Core(_) => unreachable!(
                        "BitmapInfoHeader::Core must be filtered before \
                         dispatching to parse_with_info_header"
                    ),
                    crate::parser::BitmapInfoHeader::Info(
                        crate::parser::BitmapInfoHeaderInfo {
                            compression, ..
                        },
                    )
                    | crate::parser::BitmapInfoHeader::V4(
                        crate::parser::BitmapInfoHeaderV4 {
                            compression, ..
                        },
                    )
                    | crate::parser::BitmapInfoHeader::V5(
                        crate::parser::BitmapInfoHeaderV5 {
                            compression, ..
                        },
                    ) => match compression {
                        crate::parser::Compression::BI_RGB => {
                            let bytes = Self::skip_color_usage(
                                buf,
                                color_usage,
                                dib_header_info.color_used() as usize,
                            )?;

                            Ok((Colors::Null, bytes))
                        }
                        crate::parser::Compression::BI_BITFIELDS => {
                            Self::parse_from_color_usage(
                                buf,
                                color_usage,
                                dib_header_info.color_used() as usize,
                                false,
                            )
                        }
                        _ => Ok((Colors::Null, 0)),
                    },
                }
            }
        }
    }

    /// Reads `colors_length` color-table entries and discards the parsed
    /// values, returning only the byte count. Used when the header asks
    /// the parser to walk past a color table that the caller has no use
    /// for (`BI_BITCOUNT_5` and `BI_RGB` paths).
    fn skip_color_usage<R: crate::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
        colors_length: usize,
    ) -> Result<usize, crate::parser::ParseError> {
        let (_, bytes) = Self::parse_from_color_usage(
            buf,
            color_usage,
            colors_length,
            false,
        )?;
        Ok(bytes)
    }

    fn parse_from_color_usage<R: crate::Read>(
        buf: &mut R,
        color_usage: crate::parser::ColorUsage,
        colors_length: usize,
        core: bool,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;

        match color_usage {
            crate::parser::ColorUsage::DIB_RGB_COLORS if core => {
                let mut table = Vec::with_capacity(colors_length);

                for _ in 0..colors_length {
                    let v = read_with(
                        buf,
                        &mut consumed_bytes,
                        crate::parser::RGBTriple::parse,
                    )?;
                    table.push(v);
                }

                Ok((Colors::RGBTriple(table), consumed_bytes))
            }
            crate::parser::ColorUsage::DIB_RGB_COLORS => {
                let mut table = Vec::with_capacity(colors_length);

                for _ in 0..colors_length {
                    let v = read_with(
                        buf,
                        &mut consumed_bytes,
                        crate::parser::RGBQuad::parse,
                    )?;
                    table.push(v);
                }

                Ok((Colors::RGBQuad(table), consumed_bytes))
            }
            crate::parser::ColorUsage::DIB_PAL_COLORS => {
                let mut table = Vec::with_capacity(colors_length);

                for _ in 0..colors_length {
                    let v = read_field(buf, &mut consumed_bytes)?;
                    table.push(v);
                }

                Ok((Colors::PaletteIndices(table), consumed_bytes))
            }
            // `Colors::parse` returns `Colors::Null` for this color usage
            // before any path reaches `parse_from_color_usage`.
            crate::parser::ColorUsage::DIB_PAL_INDICES => unreachable!(
                "DIB_PAL_INDICES must be handled by Colors::parse before \
                 dispatching to parse_from_color_usage"
            ),
        }
    }
}

/// The MS-WMF spec describes an optional `UndefinedSpace` slice between the
/// color table and `aData`. Its length cannot be derived from the record,
/// so this implementation never reads or stores those bytes; the field is
/// intentionally absent here. If they ever need to surface for debugging,
/// reintroduce them as `Option<Vec<u8>>` to make the "not parsed" state
/// explicit instead of conflating it with an empty buffer.
#[derive(Clone)]
pub struct BitmapBuffer {
    // /// UndefinedSpace (variable): An optional field that MUST be ignored.
    // /// If this DIB is a packed bitmap, this field MUST NOT be present.
    // pub undefined_space: Vec<u8>,
    /// aData (variable): An array of bytes that define the image.
    ///
    /// The size and format of this data is determined by information in the
    /// DIBHeaderInfo field. If it is a BitmapCoreHeader, the size in bytes
    /// MUST be calculated as follows:
    ///
    /// ```text
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

impl core::fmt::Debug for BitmapBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BitmapBuffer")
            .field("a_data", &format!("[u8; {}]", self.a_data.len()))
            .finish()
    }
}
