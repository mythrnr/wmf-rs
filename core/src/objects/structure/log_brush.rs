/// The LogBrush Object defines the style, color, and pattern of a brush. This
/// object is used only in the META_CREATEBRUSHINDIRECT Record to create a Brush
/// Object.
#[derive(Clone, Debug)]
pub enum LogBrush {
    DIBPattern,
    DIBPatternPT,
    Hatched { color_ref: crate::ColorRef, brush_hatch: crate::HatchStyle },
    Pattern,
    Solid { color_ref: crate::ColorRef },
    Null,
}

impl LogBrush {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let (style, mut consumed_bytes) = crate::BrushStyle::parse(buf)?;
        let v = match style {
            crate::BrushStyle::BS_DIBPATTERN => {
                let (_, c) = crate::read::<R, 6>(buf)?;
                consumed_bytes += c;

                Self::DIBPattern
            }
            crate::BrushStyle::BS_DIBPATTERNPT => {
                let (_, c) = crate::read::<R, 6>(buf)?;
                consumed_bytes += c;

                Self::DIBPatternPT
            }
            crate::BrushStyle::BS_HATCHED => {
                let (
                    (color_ref, color_ref_bytes),
                    (brush_hatch, brush_hatch_bytes),
                ) = (
                    crate::ColorRef::parse(buf)?,
                    crate::HatchStyle::parse(buf)?,
                );
                consumed_bytes += color_ref_bytes + brush_hatch_bytes;

                Self::Hatched { color_ref, brush_hatch }
            }
            crate::BrushStyle::BS_PATTERN => {
                let (_, c) = crate::read::<R, 6>(buf)?;
                consumed_bytes += c;

                Self::Pattern
            }
            crate::BrushStyle::BS_SOLID => {
                let ((color_ref, color_ref_bytes), (_, c)) =
                    (crate::ColorRef::parse(buf)?, crate::read::<R, 2>(buf)?);
                consumed_bytes += color_ref_bytes + c;

                Self::Solid { color_ref }
            }
            crate::BrushStyle::BS_NULL => {
                let (_, c) = crate::read::<R, 6>(buf)?;
                consumed_bytes += c;

                Self::Null
            }
            v => {
                return Err(crate::ParseError::NotSupported {
                    cause: format!("BrushStyle {v:?}"),
                });
            }
        };

        Ok((v, consumed_bytes))
    }
}
