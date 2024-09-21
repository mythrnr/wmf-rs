/// The Brush Object defines the style, color, and pattern of a brush. Brush
/// Objects are created by the META_CREATEBRUSHINDIRECT, META_CREATEPATTERNBRUSH
/// and META_DIBCREATEPATTERNBRUSH records.
#[derive(Clone, Debug)]
pub enum Brush {
    DIBPatternPT {
        color_usage: crate::parser::ColorUsage,
        brush_hatch: crate::parser::DeviceIndependentBitmap,
    },
    Hatched {
        color_ref: crate::parser::ColorRef,
        brush_hatch: crate::parser::HatchStyle,
    },
    Pattern {
        brush_hatch: crate::parser::Bitmap16,
    },
    Solid {
        color_ref: crate::parser::ColorRef,
    },
    Null,
}

impl Brush {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (style, mut consumed_bytes) = crate::parser::BrushStyle::parse(buf)?;
        let v = match style {
            crate::parser::BrushStyle::BS_DIBPATTERNPT => {
                let (color_usage, c) = crate::parser::ColorUsage::parse(buf)?;
                consumed_bytes += c;

                let (brush_hatch, c) =
                    crate::parser::DeviceIndependentBitmap::parse_with_color_usage(
                        buf,
                        color_usage,
                    )?;
                consumed_bytes += c;

                Self::DIBPatternPT { color_usage, brush_hatch }
            }
            crate::parser::BrushStyle::BS_HATCHED => {
                let (color_ref, c) = crate::parser::ColorRef::parse(buf)?;
                consumed_bytes += c;

                let (brush_hatch, c) = crate::parser::HatchStyle::parse(buf)?;
                consumed_bytes += c;

                Self::Hatched { color_ref, brush_hatch }
            }
            crate::parser::BrushStyle::BS_PATTERN => {
                // SHOULD be ignored.
                let (_, c) = crate::parser::read::<R, 4>(buf)?;
                consumed_bytes += c;

                let (brush_hatch, c) = crate::parser::Bitmap16::parse(buf)?;
                consumed_bytes += c;

                Self::Pattern { brush_hatch }
            }
            crate::parser::BrushStyle::BS_SOLID => {
                let (color_ref, c) = crate::parser::ColorRef::parse(buf)?;
                consumed_bytes += c;

                Self::Solid { color_ref }
            }
            crate::parser::BrushStyle::BS_NULL => {
                // SHOULD be ignored.
                let (_, c) = crate::parser::read::<R, 4>(buf)?;
                consumed_bytes += c;

                Self::Null
            }
            v => {
                return Err(crate::parser::ParseError::NotSupported {
                    cause: format!("BrushStyle {v:?}"),
                });
            }
        };

        Ok((v, consumed_bytes))
    }
}
