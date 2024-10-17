/// The META_CREATEBRUSHINDIRECT Record creates a Brush Object from a LogBrush
/// Object.
#[derive(Clone, Debug)]
pub struct META_CREATEBRUSHINDIRECT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_CREATEBRUSHINDIRECT.
    pub record_function: u16,
    /// LogBrush (8 bytes): LogBrush Object data that defines the brush to
    /// create. The BrushStyle field specified in the LogBrush Object SHOULD be
    /// BS_SOLID, BS_NULL, or BS_HATCHED; otherwise, a default Brush Object MAY
    /// be created. See the following table for details.
    pub log_brush: crate::parser::LogBrush,
}

impl META_CREATEBRUSHINDIRECT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_CREATEBRUSHINDIRECT,
        )?;

        let (log_brush, log_brush_bytes) = crate::parser::LogBrush::parse(buf)?;
        record_size.consume(log_brush_bytes);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, log_brush })
    }

    pub fn create_brush(&self) -> crate::parser::Brush {
        match self.log_brush.clone() {
            crate::parser::LogBrush::DIBPatternPT => {
                crate::parser::Brush::Solid {
                    color_ref: crate::parser::ColorRef::black(),
                }
            }
            crate::parser::LogBrush::Hatched { color_ref, brush_hatch } => {
                crate::parser::Brush::Hatched { color_ref, brush_hatch }
            }
            crate::parser::LogBrush::Solid { color_ref } => {
                crate::parser::Brush::Solid { color_ref }
            }
            crate::parser::LogBrush::Null => crate::parser::Brush::Null,
            _ => crate::parser::Brush::Solid {
                color_ref: crate::parser::ColorRef::black(),
            },
        }
    }
}
