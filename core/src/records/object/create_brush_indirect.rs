/// The META_CREATEBRUSHINDIRECT Record creates a Brush Object from a LogBrush
/// Object.
#[derive(Clone, Debug)]
pub struct META_CREATEBRUSHINDIRECT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_CREATEBRUSHINDIRECT.
    pub record_function: u16,
    /// LogBrush (8 bytes): LogBrush Object data that defines the brush to
    /// create. The BrushStyle field specified in the LogBrush Object SHOULD be
    /// BS_SOLID, BS_NULL, or BS_HATCHED; otherwise, a default Brush Object MAY
    /// be created. See the following table for details.
    pub log_brush: crate::LogBrush,
}

impl META_CREATEBRUSHINDIRECT {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
        mut record_size: crate::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::ParseError> {
        crate::records::check_lower_byte_matches(
            record_function,
            crate::RecordType::META_CREATEBRUSHINDIRECT,
        )?;

        let (log_brush, log_brush_bytes) = crate::LogBrush::parse(buf)?;
        record_size.consume(log_brush_bytes);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, log_brush })
    }
}

impl META_CREATEBRUSHINDIRECT {
    pub fn create_brush(&self) -> crate::Brush {
        match self.log_brush.clone() {
            crate::LogBrush::DIBPatternPT => {
                crate::Brush::Solid { color_ref: crate::ColorRef::black() }
            }
            crate::LogBrush::Hatched { color_ref, brush_hatch } => {
                crate::Brush::Hatched { color_ref, brush_hatch }
            }
            crate::LogBrush::Solid { color_ref } => {
                crate::Brush::Solid { color_ref }
            }
            crate::LogBrush::Null => crate::Brush::Null,
            _ => crate::Brush::Solid { color_ref: crate::ColorRef::black() },
        }
    }
}
