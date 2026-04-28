/// The META_HEADER Record is the first record in a standard (nonplaceable) WMF
/// metafile.
#[derive(Clone, Debug)]
pub struct META_HEADER {
    /// Type (2 bytes): A 16-bit unsigned integer that defines the type of
    /// metafile. It MUST be a value in the MetafileType Enumeration.
    pub typ: crate::parser::MetafileType,
    /// HeaderSize (2 bytes): A 16-bit unsigned integer that defines the number
    /// of 16-bit WORD structures, defined in [MS-DTYP] section 2.2.61, in the
    /// header.
    pub header_size: u16,
    /// Version (2 bytes): A 16-bit unsigned integer that defines the metafile
    /// version. It MUST be a value in the MetafileVersion Enumeration.
    /// (Metafiles created by Windows contain the value METAVERSION300.)
    pub version: crate::parser::MetafileVersion,
    /// SizeLow (2 bytes): A 16-bit unsigned integer that defines the low-order
    /// word of the number of 16-bit WORD structures in the entire metafile.
    pub size_low: u16,
    /// SizeHigh (2 bytes): A 16-bit unsigned integer that defines the
    /// high-order word of the number of 16-bit WORD structures in the entire
    /// metafile.
    pub size_high: u16,
    /// NumberOfObjects (2 bytes): A 16-bit unsigned integer that specifies the
    /// number of graphics objects that are defined in the entire metafile.
    /// These objects include brushes, pens, and the other objects specified in
    /// section 2.2.1.
    pub number_of_objects: u16,
    /// MaxRecord (4 bytes): A 32-bit unsigned integer that specifies the size
    /// of the largest record used in the metafile (in 16-bit elements).
    pub max_record: u32,
    /// NumberOfMembers (2 bytes): A 16-bit unsigned integer that is not used.
    /// It SHOULD be 0x0000.
    pub number_of_members: u16,
}

impl META_HEADER {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        fields(key = %format!("{key:#010X}")),
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub(in crate::parser::records::control) fn parse<R: crate::Read>(
        buf: &mut R,
        key: u32,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        // The 4-byte `key` value passed in by the caller already accounts
        // for the bytes read upstream; only the typ portion contributes to
        // `consumed_bytes` (matching the previous behavior where the
        // header_size byte count was discarded as it is not external).
        let bytes: [u8; 4] = key.to_le_bytes();
        let mut b = &bytes[..];
        let mut consumed_bytes: usize = 0;
        let typ = read_with(
            &mut b,
            &mut consumed_bytes,
            crate::parser::MetafileType::parse,
        )?;
        // `header_size` is read from the in-memory `key` buffer, so its
        // byte count must not be added to the externally reported
        // `consumed_bytes`.
        let mut throwaway: usize = 0;
        let header_size = read_field(&mut b, &mut throwaway)?;

        let version = read_with(
            buf,
            &mut consumed_bytes,
            crate::parser::MetafileVersion::parse,
        )?;
        let size_low = read_field(buf, &mut consumed_bytes)?;
        let size_high = read_field(buf, &mut consumed_bytes)?;
        let number_of_objects = read_field(buf, &mut consumed_bytes)?;
        let max_record = read_field(buf, &mut consumed_bytes)?;
        let number_of_members = read_field(buf, &mut consumed_bytes)?;

        if number_of_members != 0x0000 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: "The number_of_members field should be `0x0000`".into(),
            });
        }

        Ok((
            Self {
                typ,
                header_size,
                version,
                size_low,
                size_high,
                number_of_objects,
                max_record,
                number_of_members,
            },
            consumed_bytes,
        ))
    }
}
