/// The META_INTERSECTCLIPRECT Record sets the clipping region in the playback
/// device context to the intersection of the existing clipping region and the
/// specified rectangle.
#[derive(Clone, Debug)]
pub struct META_INTERSECTCLIPRECT {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::parser::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_INTERSECTCLIPRECT.
    pub record_function: u16,
    /// Bottom (2 bytes): A 16-bit signed integer that defines the
    /// y-coordinate, in logical units, of the lower-right corner of the
    /// rectangle.
    pub bottom: i16,
    /// Right (2 bytes): A 16-bit signed integer that defines the x-coordinate,
    /// in logical units, of the lower- right corner of the rectangle.
    pub right: i16,
    /// Top (2 bytes): A 16-bit signed integer that defines the y-coordinate,
    /// in logical units, of the upper-left corner of the rectangle.
    pub top: i16,
    /// Left (2 bytes): A 16-bit signed integer that defines the x-coordinate,
    /// in logical units, of the upper-left corner of the rectangle.
    pub left: i16,
}

impl META_INTERSECTCLIPRECT {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(
            %record_size,
            record_function = %crate::parser::HexU16(record_function),
        ),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
        mut record_size: crate::parser::RecordSize,
        record_function: u16,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::records::read_field;

        crate::parser::records::check_lower_byte_matches(
            record_function,
            crate::parser::RecordType::META_INTERSECTCLIPRECT,
        )?;

        let bottom = read_field(buf, &mut record_size)?;
        let right = read_field(buf, &mut record_size)?;
        let top = read_field(buf, &mut record_size)?;
        let left = read_field(buf, &mut record_size)?;

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, bottom, right, top, left })
    }
}
