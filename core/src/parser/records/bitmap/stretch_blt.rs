/// The META_STRETCHBLT Record specifies the transfer of a block of pixels
/// according to a raster operation, with possible expansion or contraction.
///
/// The destination of the transfer is the current output region in the playback
/// device context.
///
/// There are two forms of META_STRETCHBLT, one which specifies a bitmap as the
/// source, and the other which uses the playback device context as the source.
/// Definitions follow for the fields that are the same in the two forms of
/// META_STRETCHBLT are defined below. The subsections that follow specify the
/// packet structures of the two forms of META_STRETCHBLT.
///
/// The expansion or contraction is performed according to the stretching mode
/// currently set in the playback device context, which MUST be a value from the
/// StretchMode Enumeration.
///
/// The RecordSize and RecordFunction fields SHOULD be used to differentiate
/// between the two forms of META_STRETCHBLT. If the following Boolean
/// expression is TRUE, a source bitmap is not specified in the record.
///
/// ```
/// RecordSize == ((RecordFunction >> 8) + 3)
/// ```
#[derive(Clone, Debug)]
pub enum META_STRETCHBLT {
    WithBitmap {
        /// RecordSize: A 32-bit unsigned integer that defines the number of
        /// 16-bit WORD structures, defined in [MS-DTYP] section 2.2.61, in the
        /// record.
        record_size: crate::parser::RecordSize,
        /// RecordFunction: A 16-bit unsigned integer that defines this WMF
        /// record type. The low-order byte MUST match the low-order byte of
        /// the RecordType Enumeration table value META_STRETCHBLT.
        record_function: u16,
        /// RasterOperation: A 32-bit unsigned integer that defines how the
        /// source pixels, the current brush in the playback device context,
        /// and the destination pixels are to be combined to form the new
        /// image. This code MUST be one of the values in the Ternary Raster
        /// Operation Enumeration.
        raster_operation: crate::parser::TernaryRasterOperation,
        /// SrcHeight: A 16-bit signed integer that defines the height, in
        /// logical units, of the source rectangle.
        src_height: i16,
        /// SrcWidth: A 16-bit signed integer that defines the width, in
        /// logical units, of the source rectangle.
        src_width: i16,
        /// YSrc: A 16-bit signed integer that defines the y-coordinate, in
        /// logical units, of the upper-left corner of the source rectangle.
        y_src: i16,
        /// XSrc: A 16-bit signed integer that defines the x-coordinate, in
        /// logical units, of the upper-left corner of the source rectangle.
        x_src: i16,
        /// DestHeight: A 16-bit signed integer that defines the height, in
        /// logical units, of the destination rectangle.
        dest_height: i16,
        /// DestWidth: A 16-bit signed integer that defines the width, in
        /// logical units, of the destination rectangle.
        dest_width: i16,
        /// YDest: A 16-bit signed integer that defines the y-coordinate, in
        /// logical units, of the upper-left corner of the destination
        /// rectangle.
        y_dest: i16,
        /// XDest: A 16-bit signed integer that defines the x-coordinate, in
        /// logical units, of the upper-left corner of the destination
        /// rectangle.
        x_dest: i16,
        /// Target (variable): A variable-sized Bitmap16 Object that defines
        /// source image content. This object MUST be specified, even if the
        /// raster operation does not require a source.
        target: crate::parser::Bitmap16,
    },
    WithoutBitmap {
        /// RecordSize: A 32-bit unsigned integer that defines the number of
        /// 16-bit WORD structures, defined in [MS-DTYP] section 2.2.61, in the
        /// record.
        record_size: crate::parser::RecordSize,
        /// RecordFunction: A 16-bit unsigned integer that defines this WMF
        /// record type. The low-order byte MUST match the low-order byte of
        /// the RecordType Enumeration table value META_STRETCHBLT.
        record_function: u16,
        /// RasterOperation: A 32-bit unsigned integer that defines how the
        /// source pixels, the current brush in the playback device context,
        /// and the destination pixels are to be combined to form the new
        /// image. This code MUST be one of the values in the Ternary Raster
        /// Operation Enumeration.
        raster_operation: crate::parser::TernaryRasterOperation,
        /// SrcHeight: A 16-bit signed integer that defines the height, in
        /// logical units, of the source rectangle.
        src_height: i16,
        /// SrcWidth: A 16-bit signed integer that defines the width, in
        /// logical units, of the source rectangle.
        src_width: i16,
        /// YSrc: A 16-bit signed integer that defines the y-coordinate, in
        /// logical units, of the upper-left corner of the source rectangle.
        y_src: i16,
        /// XSrc: A 16-bit signed integer that defines the x-coordinate, in
        /// logical units, of the upper-left corner of the source rectangle.
        x_src: i16,
        /// Reserved (2 bytes): This field MUST be ignored.
        reserved: [u8; 2],
        /// DestHeight: A 16-bit signed integer that defines the height, in
        /// logical units, of the destination rectangle.
        dest_height: i16,
        /// DestWidth: A 16-bit signed integer that defines the width, in
        /// logical units, of the destination rectangle.
        dest_width: i16,
        /// YDest: A 16-bit signed integer that defines the y-coordinate, in
        /// logical units, of the upper-left corner of the destination
        /// rectangle.
        y_dest: i16,
        /// XDest: A 16-bit signed integer that defines the x-coordinate, in
        /// logical units, of the upper-left corner of the destination
        /// rectangle.
        x_dest: i16,
    },
}

impl META_STRETCHBLT {
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
            crate::parser::RecordType::META_STRETCHBLT,
        )?;

        let (
            (raster_operation, raster_operation_bytes),
            (src_height, src_height_bytes),
            (src_width, src_width_bytes),
            (y_src, y_src_bytes),
            (x_src, x_src_bytes),
        ) = (
            crate::parser::TernaryRasterOperation::parse(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(
            raster_operation_bytes
                + src_height_bytes
                + src_width_bytes
                + y_src_bytes
                + x_src_bytes,
        );

        let bitmap_specified =
            u32::from(record_size) != u32::from((record_function >> 8) + 3);
        let reserved = if bitmap_specified {
            [0; 2]
        } else {
            let (v, c) = crate::parser::read::<R, 2>(buf)?;
            record_size.consume(c);
            v
        };
        let (
            (dest_height, dest_height_bytes),
            (dest_width, dest_width_bytes),
            (y_dest, y_dest_bytes),
            (x_dest, x_dest_bytes),
        ) = (
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
            crate::parser::read_i16_from_le_bytes(buf)?,
        );
        record_size.consume(
            dest_height_bytes + dest_width_bytes + y_dest_bytes + x_dest_bytes,
        );

        let record = if bitmap_specified {
            let (target, c) = crate::parser::Bitmap16::parse(buf)?;
            record_size.consume(c);

            Self::WithBitmap {
                record_size,
                record_function,
                raster_operation,
                src_height,
                src_width,
                y_src,
                x_src,
                dest_height,
                dest_width,
                y_dest,
                x_dest,
                target,
            }
        } else {
            Self::WithoutBitmap {
                record_size,
                record_function,
                raster_operation,
                src_height,
                src_width,
                y_src,
                x_src,
                reserved,
                dest_height,
                dest_width,
                y_dest,
                x_dest,
            }
        };

        crate::parser::records::consume_remaining_bytes(buf, record_size)?;

        Ok(record)
    }
}
