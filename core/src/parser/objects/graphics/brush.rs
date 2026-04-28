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
            crate::parser::BrushStyle::BS_DIBPATTERNPT => {
                use crate::parser::DeviceIndependentBitmap;

                let (color_usage, c) = crate::parser::ColorUsage::parse(buf)?;
                consumed_bytes += c;

                let (brush_hatch, c) =
                    DeviceIndependentBitmap::parse_with_color_usage(
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
    fn parse_solid_brush() {
        // BS_SOLID = 0x0000 (u16) + ColorRef (4 bytes).
        let mut data = Vec::new();
        data.extend_from_slice(&0x0000_u16.to_le_bytes());
        data.extend_from_slice(&[0x12, 0x34, 0x56, 0x00]);
        let mut reader = &data[..];
        let (b, consumed) = Brush::parse(&mut reader).unwrap();
        let Brush::Solid { color_ref } = b else {
            panic!("expected Solid, got {b:?}");
        };
        assert_eq!(color_ref.red, 0x12);
        assert_eq!(consumed, 6);
    }

    #[test]
    fn parse_null_brush() {
        // BS_NULL = 0x0001 + 4 bytes filler.
        let mut data = Vec::new();
        data.extend_from_slice(&0x0001_u16.to_le_bytes());
        data.extend_from_slice(&[0u8; 4]);
        let mut reader = &data[..];
        let (b, _) = Brush::parse(&mut reader).unwrap();
        assert!(matches!(b, Brush::Null));
    }

    #[test]
    fn parse_hatched_brush() {
        // BS_HATCHED = 0x0002 + ColorRef (4 bytes) + HatchStyle (u16,
        // HS_HORIZONTAL = 0x0000).
        let mut data = Vec::new();
        data.extend_from_slice(&0x0002_u16.to_le_bytes());
        data.extend_from_slice(&[0x10, 0x20, 0x30, 0x00]);
        data.extend_from_slice(&0x0000_u16.to_le_bytes());
        let mut reader = &data[..];
        let (b, consumed) = Brush::parse(&mut reader).unwrap();
        assert!(matches!(b, Brush::Hatched { .. }));
        assert_eq!(consumed, 8);
    }

    #[test]
    fn parse_truncated_returns_err() {
        let data = 0x0000_u16.to_le_bytes(); // BS_SOLID with no color
        let mut reader = &data[..];
        assert!(Brush::parse(&mut reader).is_err());
    }
}
