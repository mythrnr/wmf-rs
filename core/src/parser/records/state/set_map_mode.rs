/// The META_SETMAPMODE Record defines the mapping mode in the playback device
/// context.
///
/// The mapping mode defines the unit of measure used to transform page-space
/// units into device-space units, and also defines the orientation of the
/// device's x and y axes.
#[derive(Clone, Debug)]
pub struct META_SETMAPMODE {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_SETMAPMODE.
    pub record_function: u16,
    /// MapMode (2 bytes): A 16-bit unsigned integer that defines the mapping
    /// mode. This MUST be one of the values enumerated in the MapMode
    /// Enumeration table.
    pub map_mode: crate::parser::MapMode,
}

impl META_SETMAPMODE {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %format!("{record_function:#06X}"),
        ),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_SETMAPMODE,
        )?;

        let (map_mode, map_mode_bytes) = crate::parser::MapMode::parse(buf)?;
        record_size.consume(map_mode_bytes);

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, map_mode })
    }
}
