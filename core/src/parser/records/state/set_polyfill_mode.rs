/// The META_SETPOLYFILLMODE Record sets polygon fill mode in the playback
/// device context for graphics operations that fill polygons.
#[derive(Clone, Debug)]
pub struct META_SETPOLYFILLMODE {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SETPOLYFILLMODE.
    pub record_function: u16,
    /// PolyFillMode (2 bytes): A 16-bit unsigned integer that defines polygon
    /// fill mode. This MUST be one of the values in the PolyFillMode
    /// Enumeration.
    pub poly_fill_mode: crate::parser::PolyFillMode,
    /// Reserved (2 bytes): An optional 16-bit field that MUST be ignored.
    /// (Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 implementations do
    /// not write this field to WMF metafiles.)
    pub reserved: Option<[u8; 2]>,
}

impl META_SETPOLYFILLMODE {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::ERROR, Display),
    )]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_SETPOLYFILLMODE,
        )?;

        let (poly_fill_mode, poly_fill_mode_bytes) =
            crate::parser::PolyFillMode::parse(buf)?;
        record_size.consume(poly_fill_mode_bytes);

        let reserved = if record_size.byte_count() > 8 {
            let (v, c) = crate::parser::read::<R, 2>(buf)?;
            record_size.consume(c);
            Some(v)
        } else {
            None
        };

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, poly_fill_mode, reserved })
    }
}
