/// The LogBrush Object defines the style, color, and pattern of a brush. This
/// object is used only in the META_CREATEBRUSHINDIRECT Record to create a Brush
/// Object.
#[derive(Clone, Debug)]
pub enum LogBrush {
    DIBPattern,
    DIBPatternPT,
    Hatched {
        color_ref: crate::parser::ColorRef,
        brush_hatch: crate::parser::HatchStyle,
    },
    Pattern,
    Solid {
        color_ref: crate::parser::ColorRef,
    },
    Null,
}

impl LogBrush {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (style, mut consumed_bytes) =
            crate::parser::BrushStyle::parse(buf)?;
        let v = match style {
            crate::parser::BrushStyle::BS_DIBPATTERN => {
                let (_, c) = crate::parser::read::<R, 6>(buf)?;
                consumed_bytes += c;

                Self::DIBPattern
            }
            crate::parser::BrushStyle::BS_DIBPATTERNPT => {
                let (_, c) = crate::parser::read::<R, 6>(buf)?;
                consumed_bytes += c;

                Self::DIBPatternPT
            }
            crate::parser::BrushStyle::BS_HATCHED => {
                let (
                    (color_ref, color_ref_bytes),
                    (brush_hatch, brush_hatch_bytes),
                ) = (
                    crate::parser::ColorRef::parse(buf)?,
                    crate::parser::HatchStyle::parse(buf)?,
                );
                consumed_bytes += color_ref_bytes + brush_hatch_bytes;

                Self::Hatched { color_ref, brush_hatch }
            }
            crate::parser::BrushStyle::BS_PATTERN => {
                let (_, c) = crate::parser::read::<R, 6>(buf)?;
                consumed_bytes += c;

                Self::Pattern
            }
            crate::parser::BrushStyle::BS_SOLID => {
                let ((color_ref, color_ref_bytes), (_, c)) = (
                    crate::parser::ColorRef::parse(buf)?,
                    crate::parser::read::<R, 2>(buf)?,
                );
                consumed_bytes += color_ref_bytes + c;

                Self::Solid { color_ref }
            }
            crate::parser::BrushStyle::BS_NULL => {
                let (_, c) = crate::parser::read::<R, 6>(buf)?;
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
