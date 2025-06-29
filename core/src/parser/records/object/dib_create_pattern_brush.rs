/// The META_DIBCREATEPATTERNBRUSH Record creates a Brush Object with a pattern
/// specified by a DeviceIndependentBitmap (DIB) Object.
#[derive(Clone, Debug)]
pub struct META_DIBCREATEPATTERNBRUSH {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// record type. The lower byte MUST match the lower byte of the RecordType
    /// Enumeration table value META_DIBCREATEPATTERNBRUSH.
    pub record_function: u16,
    /// Style (2 bytes): A 16-bit unsigned integer that defines the brush
    /// style. The legal values for this field are defined as follows: if the
    /// value is not BS_PATTERN, BS_DIBPATTERNPT MUST be assumed.
    /// These values are specified in the BrushStyle Enumeration.
    pub style: crate::parser::BrushStyle,
    /// ColorUsage (2 bytes): A 16-bit unsigned integer that defines whether
    /// the Colors field of a DIB Object contains explicit RGB values, or
    /// indexes into a palette.
    ///
    /// If the Style field specifies BS_PATTERN, a ColorUsage value of
    /// DIB_RGB_COLORS MUST be used regardless of the contents of this field.
    ///
    /// If the Style field specified anything but BS_PATTERN, this field MUST
    /// be one of the values in the ColorUsage Enumeration.
    pub color_usage: crate::parser::ColorUsage,
    /// Target (variable): Variable-bit DIB Object data that defines the
    /// pattern to use in the brush.
    pub target: crate::parser::DeviceIndependentBitmap,
}

impl META_DIBCREATEPATTERNBRUSH {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_DIBCREATEPATTERNBRUSH,
        )?;

        let ((mut style, style_bytes), (mut color_usage, color_usage_bytes)) = (
            crate::parser::BrushStyle::parse(buf)?,
            crate::parser::ColorUsage::parse(buf)?,
        );
        record_size.consume(style_bytes + color_usage_bytes);

        if matches!(style, crate::parser::BrushStyle::BS_PATTERN) {
            color_usage = crate::parser::ColorUsage::DIB_RGB_COLORS;
        } else {
            style = crate::parser::BrushStyle::BS_DIBPATTERNPT;
        }

        let (target, c) =
            crate::parser::DeviceIndependentBitmap::parse_with_color_usage(
                buf,
                color_usage,
            )?;
        record_size.consume(c);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, style, color_usage, target })
    }

    pub fn create_brush(&self) -> crate::parser::Brush {
        match self.style {
            crate::parser::BrushStyle::BS_PATTERN => {
                crate::parser::Brush::DIBPatternPT {
                    color_usage: crate::parser::ColorUsage::DIB_RGB_COLORS,
                    brush_hatch: self.target.clone(),
                }
            }
            _ => crate::parser::Brush::DIBPatternPT {
                color_usage: self.color_usage,
                brush_hatch: self.target.clone(),
            },
        }
    }
}
