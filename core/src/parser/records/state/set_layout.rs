/// The META_SETLAYOUT Record defines the layout orientation in the playback
/// device context. (Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and
/// Windows NT 4.0 implementations do not support this record type.) The layout
/// orientation determines the direction in which text and graphics are drawn.
#[derive(Clone, Debug)]
pub struct META_SETLAYOUT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SETLAYOUT.
    pub record_function: u16,
    /// Layout (2 bytes): A 16-bit unsigned integer that defines the layout of
    /// text and graphics. This MUST be one of the values in the Layout
    /// Enumeration.
    pub layout: crate::parser::Layout,
    /// Reserved (2 bytes): A 16-bit field that MUST be ignored.
    pub reserved: [u8; 2],
}

impl META_SETLAYOUT {
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
            crate::parser::RecordType::META_SETLAYOUT,
        )?;

        let ((layout, layout_bytes), (reserved, reserved_bytes)) = (
            crate::parser::Layout::parse(buf)?,
            crate::parser::read::<R, 2>(buf)?,
        );
        record_size.consume(layout_bytes + reserved_bytes);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, layout, reserved })
    }
}
