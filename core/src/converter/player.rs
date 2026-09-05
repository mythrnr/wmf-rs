use crate::{imports::*, parser::*};

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum PlayError {
    #[snafu(display("failed to generate: {cause}"))]
    FailedGenerate { cause: String },
    #[snafu(display("invalid brush: {cause}"))]
    InvalidBrush { cause: String },
    #[snafu(display("invalid record: {cause}"))]
    InvalidRecord { cause: String },
    #[snafu(display("unexpected graphics object: {cause}"))]
    UnexpectedGraphicsObject { cause: String },
    #[snafu(display("unknown: {cause}"))]
    Unknown { cause: String },
}

/// Generates default implementations for the record handlers of
/// [`Player`].
///
/// Each generated handler logs the record as skipped and returns the
/// player unchanged. Handler names are listed explicitly because they do
/// not derive mechanically from record type names (e.g. `META_SETROP2`
/// maps to `set_raster_operation`).
macro_rules! default_record_handlers {
    ($($name:ident : $record:ident),+ $(,)?) => {
        $(
            #[doc = concat!(
                "Render [`", stringify!($record), "`](crate::parser::",
                stringify!($record), ") record."
            )]
            fn $name(
                self,
                _record_number: usize,
                _record: $record,
            ) -> Result<Self, PlayError> {
                info!(concat!(
                    stringify!($record),
                    ": skipped (not implemented)"
                ));
                Ok(self)
            }
        )+
    };
}

/// Renders parsed WMF records into an output format.
///
/// Only [`Player::generate`] and [`Player::header`] are required. Every
/// other record handler has a default implementation that logs the record
/// as skipped and returns the player unchanged, so an implementation
/// overrides only the records its output format supports.
pub trait Player: Sized {
    /// Call after converting to write output.
    fn generate(self) -> Result<Vec<u8>, PlayError>;

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    default_record_handlers! {
        bit_blt: META_BITBLT,
        device_independent_bitmap_bit_blt: META_DIBBITBLT,
        device_independent_bitmap_stretch_blt: META_DIBSTRETCHBLT,
        set_device_independent_bitmap_to_dev: META_SETDIBTODEV,
        stretch_blt: META_STRETCHBLT,
        stretch_device_independent_bitmap: META_STRETCHDIB,
    }

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    default_record_handlers! {
        eof: META_EOF,
    }

    /// Render [`MetafileHeader`](crate::parser::MetafileHeader) record.
    fn header(
        self,
        record_number: usize,
        header: MetafileHeader,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .
    default_record_handlers! {
        arc: META_ARC,
        chord: META_CHORD,
        ellipse: META_ELLIPSE,
        ext_flood_fill: META_EXTFLOODFILL,
        ext_text_out: META_EXTTEXTOUT,
        fill_region: META_FILLREGION,
        flood_fill: META_FLOODFILL,
        frame_region: META_FRAMEREGION,
        invert_region: META_INVERTREGION,
        line_to: META_LINETO,
        paint_region: META_PAINTREGION,
        pat_blt: META_PATBLT,
        pie: META_PIE,
        polyline: META_POLYLINE,
        polygon: META_POLYGON,
        poly_polygon: META_POLYPOLYGON,
        rectangle: META_RECTANGLE,
        round_rect: META_ROUNDRECT,
        set_pixel: META_SETPIXEL,
        text_out: META_TEXTOUT,
    }

    // .
    // .
    // Functions to handle Object Record
    // .
    // .
    default_record_handlers! {
        create_brush_indirect: META_CREATEBRUSHINDIRECT,
        create_font_indirect: META_CREATEFONTINDIRECT,
        create_palette: META_CREATEPALETTE,
        create_pattern_brush: META_CREATEPATTERNBRUSH,
        create_pen_indirect: META_CREATEPENINDIRECT,
        create_region: META_CREATEREGION,
        delete_object: META_DELETEOBJECT,
        create_device_independent_bitmap_pattern_brush:
            META_DIBCREATEPATTERNBRUSH,
        select_clip_region: META_SELECTCLIPREGION,
        select_object: META_SELECTOBJECT,
        select_palette: META_SELECTPALETTE,
    }

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    default_record_handlers! {
        animate_palette: META_ANIMATEPALETTE,
        exclude_clip_rect: META_EXCLUDECLIPRECT,
        intersect_clip_rect: META_INTERSECTCLIPRECT,
        move_to: META_MOVETO,
        offset_clip_region: META_OFFSETCLIPRGN,
        offset_viewport_origin: META_OFFSETVIEWPORTORG,
        offset_window_origin: META_OFFSETWINDOWORG,
        realize_palette: META_REALIZEPALETTE,
        resize_palette: META_RESIZEPALETTE,
        restore_device_context: META_RESTOREDC,
        save_device_context: META_SAVEDC,
        scale_viewport_ext: META_SCALEVIEWPORTEXT,
        scale_window_ext: META_SCALEWINDOWEXT,
        set_bk_color: META_SETBKCOLOR,
        set_bk_mode: META_SETBKMODE,
        set_layout: META_SETLAYOUT,
        set_map_mode: META_SETMAPMODE,
        set_mapper_flags: META_SETMAPPERFLAGS,
        set_pal_entries: META_SETPALENTRIES,
        set_polyfill_mode: META_SETPOLYFILLMODE,
        set_relabs: META_SETRELABS,
        set_raster_operation: META_SETROP2,
        set_stretch_blt_mode: META_SETSTRETCHBLTMODE,
        set_text_align: META_SETTEXTALIGN,
        set_text_char_extra: META_SETTEXTCHAREXTRA,
        set_text_color: META_SETTEXTCOLOR,
        set_text_justification: META_SETTEXTJUSTIFICATION,
        set_viewport_ext: META_SETVIEWPORTEXT,
        set_viewport_origin: META_SETVIEWPORTORG,
        set_window_ext: META_SETWINDOWEXT,
        set_window_origin: META_SETWINDOWORG,
    }

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .
    default_record_handlers! {
        escape: META_ESCAPE,
    }
}

#[cfg(test)]
mod tests {
    use crate::{imports::*, parser::*};

    /// Implements only the required functions. Compiling this type proves
    /// that every record handler has a default implementation.
    struct NullPlayer;

    impl super::Player for NullPlayer {
        fn generate(self) -> Result<Vec<u8>, super::PlayError> {
            Ok(Vec::new())
        }

        fn header(
            self,
            _record_number: usize,
            _header: MetafileHeader,
        ) -> Result<Self, super::PlayError> {
            Ok(self)
        }
    }

    /// Encode one record as it appears in a WMF stream: RecordSize in
    /// words, RecordFunction, then the 16-bit payload values.
    fn record(record_type: RecordType, payload: &[i16]) -> Vec<u8> {
        let word_count = payload.len() as u32 + 3;
        let mut data = Vec::new();
        data.extend_from_slice(&word_count.to_le_bytes());
        data.extend_from_slice(&(record_type as u16).to_le_bytes());
        for value in payload {
            data.extend_from_slice(&value.to_le_bytes());
        }
        data
    }

    /// Encode a standard (non-placeable) metafile header.
    fn standard_header() -> Vec<u8> {
        let mut data = Vec::new();
        // MetafileType: MEMORYMETAFILE
        data.extend_from_slice(&0x0001_u16.to_le_bytes());
        // HeaderSize: 9 words
        data.extend_from_slice(&9_u16.to_le_bytes());
        // Version: METAVERSION300
        data.extend_from_slice(&0x0300_u16.to_le_bytes());
        // SizeLow, SizeHigh
        data.extend_from_slice(&0_u32.to_le_bytes());
        // NumberOfObjects
        data.extend_from_slice(&0_u16.to_le_bytes());
        // MaxRecord
        data.extend_from_slice(&0_u32.to_le_bytes());
        // NumberOfMembers
        data.extend_from_slice(&0_u16.to_le_bytes());
        data
    }

    #[test]
    fn default_handlers_complete_conversion() {
        struct TestCase {
            record_type: RecordType,
            payload: &'static [i16],
        }

        // One record per category (control, drawing, object, state) that
        // the NullPlayer must survive on default handlers alone.
        let records = [
            TestCase { record_type: RecordType::META_SAVEDC, payload: &[] },
            TestCase {
                record_type: RecordType::META_SETWINDOWEXT,
                payload: &[200, 200],
            },
            TestCase {
                record_type: RecordType::META_MOVETO,
                payload: &[10, 10],
            },
            TestCase {
                record_type: RecordType::META_LINETO,
                payload: &[20, 30],
            },
            TestCase {
                record_type: RecordType::META_RECTANGLE,
                payload: &[100, 100, 10, 10],
            },
            TestCase {
                record_type: RecordType::META_SELECTOBJECT,
                payload: &[0],
            },
            TestCase {
                record_type: RecordType::META_DELETEOBJECT,
                payload: &[0],
            },
            TestCase {
                record_type: RecordType::META_RESTOREDC,
                payload: &[-1],
            },
            TestCase { record_type: RecordType::META_EOF, payload: &[] },
        ];

        let mut data = standard_header();
        for case in &records {
            data.extend_from_slice(&record(case.record_type, case.payload));
        }

        let converter =
            crate::converter::WMFConverter::new(&data[..], NullPlayer);
        let output = converter
            .run()
            .expect("default handlers should satisfy the whole stream");
        assert!(output.is_empty(), "NullPlayer generates empty output");
    }

    #[test]
    fn default_handler_returns_ok() {
        use super::Player;

        NullPlayer
            .save_device_context(1, META_SAVEDC {
                record_size: RecordSize::from_raw(0),
                record_function: 0,
            })
            .and_then(|player| {
                player.line_to(2, META_LINETO {
                    record_size: RecordSize::from_raw(0),
                    record_function: 0,
                    y: 10,
                    x: 20,
                })
            })
            .and_then(|player| {
                player.eof(3, META_EOF {
                    record_size: RecordSize::from_raw(0),
                    record_function: 0,
                })
            })
            .expect("default handlers should return Ok");
    }
}
