/// The META_CREATEPATTERNBRUSH Record creates a brush object with a pattern
/// specified by a bitmap. (The META_CREATEPATTERNBRUSH record type is
/// deprecated. No version of Windows emits this record. Playback implementation
/// of this record in Windows is intended solely for compatibility purposes so
/// that Windows metafiles containing this record can be rendered.)
#[derive(Clone)]
pub struct META_CREATEPATTERNBRUSH {
    /// RecordSize (4 bytes): A 32-bit unsigned integer that defines the number
    /// of WORD structures, defined in [MS-DTYP] section 2.2.61, in the WMF
    /// record.
    pub record_size: crate::RecordSize,
    /// RecordFunction (2 bytes): A 16-bit unsigned integer that defines this
    /// WMF record type. The lower byte MUST match the lower byte of the
    /// RecordType Enumeration table value META_CREATEPATTERNBRUSH.
    pub record_function: u16,
    /// Bitmap16 (14 bytes): A partial Bitmap16 Object, which defines
    /// parameters for the bitmap that specifies the pattern for the brush.
    pub bitmap16: crate::Bitmap16,
    /// Reserved (18 bytes): This field MUST be ignored.
    pub reserved: [u8; 18],
    /// Pattern (variable): A variable-length array of bytes that defines the
    /// bitmap pixel data that composes the brush pattern. The length of this
    /// field, in bytes, can be computed from bitmap parameters as follows.
    ///
    /// ```
    /// (((Width * BitsPixel + 15) >> 4) << 1) * Height
    /// ```
    pub pattern: Vec<u8>,
}

impl std::fmt::Debug for META_CREATEPATTERNBRUSH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("META_CREATEPATTERNBRUSH")
            .field("record_size", &self.record_size)
            .field("record_function", &self.record_function)
            .field("bitmap16", &self.bitmap16)
            .field("reserved", &self.reserved)
            .field("pattern", &format!("[u8; {}]", self.pattern.len()))
            .finish()
    }
}

impl META_CREATEPATTERNBRUSH {
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
            crate::RecordType::META_CREATEPATTERNBRUSH,
        )?;

        let ((bitmap16, bitmap16_bytes), (reserved, reserved_bytes)) =
            (crate::Bitmap16::parse(buf)?, crate::read::<R, 18>(buf)?);
        record_size.consume(bitmap16_bytes + reserved_bytes);

        let length = (((bitmap16.width * i16::from(bitmap16.bits_pixel) + 15)
            >> 4)
            << 1)
            * bitmap16.height;
        let (pattern, c) = crate::read_variable(buf, length as usize)?;
        record_size.consume(c);

        crate::records::consume_remaining_bytes(buf, record_size)?;

        Ok(Self { record_size, record_function, bitmap16, reserved, pattern })
    }
}
