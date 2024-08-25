/// The META_SETROP2 Record defines the foreground raster operation mix mode in
/// the playback device context. The foreground mix mode is the mode for
/// combining pens and interiors of filled objects with foreground colors on the
/// output surface.
#[derive(Clone, Debug)]
pub struct META_SETROP2 {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SETROP2.
    pub record_function: u16,
    /// DrawMode (2 bytes): A 16-bit unsigned integer that defines the
    /// foreground binary raster operation mixing mode. This MUST be one of the
    /// values in the BinaryRasterOperation Enumeration.
    pub draw_mode: crate::BinaryRasterOperation,
    /// Reserved (2 bytes): An optional 16-bit field that MUST be ignored.
    /// (Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 implementations do
    /// not write this field to WMF metafiles.)
    pub reserved: Option<[u8; 2]>,
}

impl META_SETROP2 {
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
            crate::RecordType::META_SETROP2,
        )?;

        let (draw_mode, draw_mode_bytes) =
            crate::BinaryRasterOperation::parse(buf)?;
        record_size.consume(draw_mode_bytes);

        let reserved = if record_size.byte_count() > 8 {
            let (v, c) = crate::read::<R, 2>(buf)?;

            record_size.consume(c);
            Some(v)
        } else {
            None
        };

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, draw_mode, reserved })
    }
}
