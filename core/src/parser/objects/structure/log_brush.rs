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
                    cause: format!("BrushStyle {v:?}").into(),
                });
            }
        };

        Ok((v, consumed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imports::*;

    #[test]
    fn parse_solid() {
        // BrushStyle: BS_SOLID = 0x0000 (u16)
        // ColorRef: 4 bytes (RGB + reserved)
        // Plus 2 trailing bytes consumed by the BS_SOLID branch.
        let mut data = Vec::new();
        data.extend_from_slice(&0x0000_u16.to_le_bytes()); // BS_SOLID
        data.extend_from_slice(&[0x10, 0x20, 0x30, 0x00]); // ColorRef
        data.extend_from_slice(&[0x00, 0x00]); // brush_hatch placeholder
        let mut reader = &data[..];
        let (b, consumed) = LogBrush::parse(&mut reader).unwrap();
        let LogBrush::Solid { color_ref } = b else {
            panic!("expected Solid variant, got {b:?}");
        };
        assert_eq!(color_ref.red, 0x10);
        assert_eq!(color_ref.green, 0x20);
        assert_eq!(color_ref.blue, 0x30);
        assert_eq!(consumed, 8);
    }

    #[test]
    fn parse_null() {
        // BS_NULL = 0x0001 + 6 bytes filler.
        let mut data = Vec::new();
        data.extend_from_slice(&0x0001_u16.to_le_bytes());
        data.extend_from_slice(&[0u8; 6]);
        let mut reader = &data[..];
        let (b, consumed) = LogBrush::parse(&mut reader).unwrap();
        assert!(matches!(b, LogBrush::Null));
        assert_eq!(consumed, 8);
    }

    #[test]
    fn parse_truncated() {
        // Style only, no payload bytes following.
        let data = 0x0000_u16.to_le_bytes();
        let mut reader = &data[..];
        assert!(LogBrush::parse(&mut reader).is_err());
    }
}
