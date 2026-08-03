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

    /// Render [`META_BITBLT`](crate::parser::META_BITBLT) record.
    fn bit_blt(
        self,
        _record_number: usize,
        _record: META_BITBLT,
    ) -> Result<Self, PlayError> {
        info!("META_BITBLT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_DIBBITBLT`](crate::parser::META_DIBBITBLT) record.
    fn device_independent_bitmap_bit_blt(
        self,
        _record_number: usize,
        _record: META_DIBBITBLT,
    ) -> Result<Self, PlayError> {
        info!("META_DIBBITBLT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_DIBSTRETCHBLT`](crate::parser::META_DIBSTRETCHBLT) record.
    fn device_independent_bitmap_stretch_blt(
        self,
        _record_number: usize,
        _record: META_DIBSTRETCHBLT,
    ) -> Result<Self, PlayError> {
        info!("META_DIBSTRETCHBLT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETDIBTODEV`](crate::parser::META_SETDIBTODEV) record.
    fn set_device_independent_bitmap_to_dev(
        self,
        _record_number: usize,
        _record: META_SETDIBTODEV,
    ) -> Result<Self, PlayError> {
        info!("META_SETDIBTODEV: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_STRETCHBLT`](crate::parser::META_STRETCHBLT) record.
    fn stretch_blt(
        self,
        _record_number: usize,
        _record: META_STRETCHBLT,
    ) -> Result<Self, PlayError> {
        info!("META_STRETCHBLT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_STRETCHDIB`](crate::parser::META_STRETCHDIB) record.
    fn stretch_device_independent_bitmap(
        self,
        _record_number: usize,
        _record: META_STRETCHDIB,
    ) -> Result<Self, PlayError> {
        info!("META_STRETCHDIB: skipped (not implemented)");
        Ok(self)
    }

    // .
    // .
    // Functions to handle Control Record
    // .
    // .

    /// Render [`META_EOF`](crate::parser::META_EOF) record.
    fn eof(
        self,
        _record_number: usize,
        _record: META_EOF,
    ) -> Result<Self, PlayError> {
        info!("META_EOF: skipped (not implemented)");
        Ok(self)
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

    /// Render [`META_ARC`](crate::parser::META_ARC) record.
    fn arc(
        self,
        _record_number: usize,
        _record: META_ARC,
    ) -> Result<Self, PlayError> {
        info!("META_ARC: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_CHORD`](crate::parser::META_CHORD) record.
    fn chord(
        self,
        _record_number: usize,
        _record: META_CHORD,
    ) -> Result<Self, PlayError> {
        info!("META_CHORD: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_ELLIPSE`](crate::parser::META_ELLIPSE) record.
    fn ellipse(
        self,
        _record_number: usize,
        _record: META_ELLIPSE,
    ) -> Result<Self, PlayError> {
        info!("META_ELLIPSE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_EXTFLOODFILL`](crate::parser::META_EXTFLOODFILL) record.
    fn ext_flood_fill(
        self,
        _record_number: usize,
        _record: META_EXTFLOODFILL,
    ) -> Result<Self, PlayError> {
        info!("META_EXTFLOODFILL: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_EXTTEXTOUT`](crate::parser::META_EXTTEXTOUT) record.
    fn ext_text_out(
        self,
        _record_number: usize,
        _record: META_EXTTEXTOUT,
    ) -> Result<Self, PlayError> {
        info!("META_EXTTEXTOUT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_FILLREGION`](crate::parser::META_FILLREGION) record.
    fn fill_region(
        self,
        _record_number: usize,
        _record: META_FILLREGION,
    ) -> Result<Self, PlayError> {
        info!("META_FILLREGION: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_FLOODFILL`](crate::parser::META_FLOODFILL) record.
    fn flood_fill(
        self,
        _record_number: usize,
        _record: META_FLOODFILL,
    ) -> Result<Self, PlayError> {
        info!("META_FLOODFILL: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_FRAMEREGION`](crate::parser::META_FRAMEREGION) record.
    fn frame_region(
        self,
        _record_number: usize,
        _record: META_FRAMEREGION,
    ) -> Result<Self, PlayError> {
        info!("META_FRAMEREGION: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_INVERTREGION`](crate::parser::META_INVERTREGION) record.
    fn invert_region(
        self,
        _record_number: usize,
        _record: META_INVERTREGION,
    ) -> Result<Self, PlayError> {
        info!("META_INVERTREGION: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_LINETO`](crate::parser::META_LINETO) record.
    fn line_to(
        self,
        _record_number: usize,
        _record: META_LINETO,
    ) -> Result<Self, PlayError> {
        info!("META_LINETO: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_PAINTREGION`](crate::parser::META_PAINTREGION) record.
    fn paint_region(
        self,
        _record_number: usize,
        _record: META_PAINTREGION,
    ) -> Result<Self, PlayError> {
        info!("META_PAINTREGION: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_PATBLT`](crate::parser::META_PATBLT) record.
    fn pat_blt(
        self,
        _record_number: usize,
        _record: META_PATBLT,
    ) -> Result<Self, PlayError> {
        info!("META_PATBLT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_PIE`](crate::parser::META_PIE) record.
    fn pie(
        self,
        _record_number: usize,
        _record: META_PIE,
    ) -> Result<Self, PlayError> {
        info!("META_PIE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_POLYLINE`](crate::parser::META_POLYLINE) record.
    fn polyline(
        self,
        _record_number: usize,
        _record: META_POLYLINE,
    ) -> Result<Self, PlayError> {
        info!("META_POLYLINE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_POLYGON`](crate::parser::META_POLYGON) record.
    fn polygon(
        self,
        _record_number: usize,
        _record: META_POLYGON,
    ) -> Result<Self, PlayError> {
        info!("META_POLYGON: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_POLYPOLYGON`](crate::parser::META_POLYPOLYGON) record.
    fn poly_polygon(
        self,
        _record_number: usize,
        _record: META_POLYPOLYGON,
    ) -> Result<Self, PlayError> {
        info!("META_POLYPOLYGON: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_RECTANGLE`](crate::parser::META_RECTANGLE) record.
    fn rectangle(
        self,
        _record_number: usize,
        _record: META_RECTANGLE,
    ) -> Result<Self, PlayError> {
        info!("META_RECTANGLE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_ROUNDRECT`](crate::parser::META_ROUNDRECT) record.
    fn round_rect(
        self,
        _record_number: usize,
        _record: META_ROUNDRECT,
    ) -> Result<Self, PlayError> {
        info!("META_ROUNDRECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETPIXEL`](crate::parser::META_SETPIXEL) record.
    fn set_pixel(
        self,
        _record_number: usize,
        _record: META_SETPIXEL,
    ) -> Result<Self, PlayError> {
        info!("META_SETPIXEL: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_TEXTOUT`](crate::parser::META_TEXTOUT) record.
    fn text_out(
        self,
        _record_number: usize,
        _record: META_TEXTOUT,
    ) -> Result<Self, PlayError> {
        info!("META_TEXTOUT: skipped (not implemented)");
        Ok(self)
    }

    // .
    // .
    // Functions to handle Object Record
    // .
    // .

    /// Render [`META_CREATEBRUSHINDIRECT`](crate::parser::META_CREATEBRUSHINDIRECT) record.
    fn create_brush_indirect(
        self,
        _record_number: usize,
        _record: META_CREATEBRUSHINDIRECT,
    ) -> Result<Self, PlayError> {
        info!("META_CREATEBRUSHINDIRECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_CREATEFONTINDIRECT`](crate::parser::META_CREATEFONTINDIRECT) record.
    fn create_font_indirect(
        self,
        _record_number: usize,
        _record: META_CREATEFONTINDIRECT,
    ) -> Result<Self, PlayError> {
        info!("META_CREATEFONTINDIRECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_CREATEPALETTE`](crate::parser::META_CREATEPALETTE) record.
    fn create_palette(
        self,
        _record_number: usize,
        _record: META_CREATEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_CREATEPALETTE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_CREATEPATTERNBRUSH`](crate::parser::META_CREATEPATTERNBRUSH) record.
    fn create_pattern_brush(
        self,
        _record_number: usize,
        _record: META_CREATEPATTERNBRUSH,
    ) -> Result<Self, PlayError> {
        info!("META_CREATEPATTERNBRUSH: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_CREATEPENINDIRECT`](crate::parser::META_CREATEPENINDIRECT)
    /// record.
    fn create_pen_indirect(
        self,
        _record_number: usize,
        _record: META_CREATEPENINDIRECT,
    ) -> Result<Self, PlayError> {
        info!("META_CREATEPENINDIRECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_CREATEREGION`](crate::parser::META_CREATEREGION) record.
    fn create_region(
        self,
        _record_number: usize,
        _record: META_CREATEREGION,
    ) -> Result<Self, PlayError> {
        info!("META_CREATEREGION: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_DELETEOBJECT`](crate::parser::META_DELETEOBJECT) record.
    fn delete_object(
        self,
        _record_number: usize,
        _record: META_DELETEOBJECT,
    ) -> Result<Self, PlayError> {
        info!("META_DELETEOBJECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_DIBCREATEPATTERNBRUSH`](crate::parser::META_DIBCREATEPATTERNBRUSH) record.
    fn create_device_independent_bitmap_pattern_brush(
        self,
        _record_number: usize,
        _record: META_DIBCREATEPATTERNBRUSH,
    ) -> Result<Self, PlayError> {
        info!("META_DIBCREATEPATTERNBRUSH: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SELECTCLIPREGION`](crate::parser::META_SELECTCLIPREGION)
    /// record.
    fn select_clip_region(
        self,
        _record_number: usize,
        _record: META_SELECTCLIPREGION,
    ) -> Result<Self, PlayError> {
        info!("META_SELECTCLIPREGION: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SELECTOBJECT`](crate::parser::META_SELECTOBJECT) record.
    fn select_object(
        self,
        _record_number: usize,
        _record: META_SELECTOBJECT,
    ) -> Result<Self, PlayError> {
        info!("META_SELECTOBJECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SELECTPALETTE`](crate::parser::META_SELECTPALETTE) record.
    fn select_palette(
        self,
        _record_number: usize,
        _record: META_SELECTPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_SELECTPALETTE: skipped (not implemented)");
        Ok(self)
    }

    // .
    // .
    // Functions to handle State Record
    // .
    // .

    /// Render [`META_ANIMATEPALETTE`](crate::parser::META_ANIMATEPALETTE)
    /// record.
    fn animate_palette(
        self,
        _record_number: usize,
        _record: META_ANIMATEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_ANIMATEPALETTE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_EXCLUDECLIPRECT`](crate::parser::META_EXCLUDECLIPRECT)
    /// record.
    fn exclude_clip_rect(
        self,
        _record_number: usize,
        _record: META_EXCLUDECLIPRECT,
    ) -> Result<Self, PlayError> {
        info!("META_EXCLUDECLIPRECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_INTERSECTCLIPRECT`](crate::parser::META_INTERSECTCLIPRECT)
    /// record.
    fn intersect_clip_rect(
        self,
        _record_number: usize,
        _record: META_INTERSECTCLIPRECT,
    ) -> Result<Self, PlayError> {
        info!("META_INTERSECTCLIPRECT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_MOVETO`](crate::parser::META_MOVETO) record.
    fn move_to(
        self,
        _record_number: usize,
        _record: META_MOVETO,
    ) -> Result<Self, PlayError> {
        info!("META_MOVETO: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_OFFSETCLIPRGN`](crate::parser::META_OFFSETCLIPRGN) record.
    fn offset_clip_region(
        self,
        _record_number: usize,
        _record: META_OFFSETCLIPRGN,
    ) -> Result<Self, PlayError> {
        info!("META_OFFSETCLIPRGN: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_OFFSETVIEWPORTORG`](crate::parser::META_OFFSETVIEWPORTORG)
    /// record.
    fn offset_viewport_origin(
        self,
        _record_number: usize,
        _record: META_OFFSETVIEWPORTORG,
    ) -> Result<Self, PlayError> {
        info!("META_OFFSETVIEWPORTORG: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_OFFSETWINDOWORG`](crate::parser::META_OFFSETWINDOWORG)
    /// record.
    fn offset_window_origin(
        self,
        _record_number: usize,
        _record: META_OFFSETWINDOWORG,
    ) -> Result<Self, PlayError> {
        info!("META_OFFSETWINDOWORG: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_REALIZEPALETTE`](crate::parser::META_REALIZEPALETTE)
    /// record.
    fn realize_palette(
        self,
        _record_number: usize,
        _record: META_REALIZEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_REALIZEPALETTE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_RESIZEPALETTE`](crate::parser::META_RESIZEPALETTE) record.
    fn resize_palette(
        self,
        _record_number: usize,
        _record: META_RESIZEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_RESIZEPALETTE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_RESTOREDC`](crate::parser::META_RESTOREDC) record.
    fn restore_device_context(
        self,
        _record_number: usize,
        _record: META_RESTOREDC,
    ) -> Result<Self, PlayError> {
        info!("META_RESTOREDC: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SAVEDC`](crate::parser::META_SAVEDC) record.
    fn save_device_context(
        self,
        _record_number: usize,
        _record: META_SAVEDC,
    ) -> Result<Self, PlayError> {
        info!("META_SAVEDC: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SCALEVIEWPORTEXT`](crate::parser::META_SCALEVIEWPORTEXT)
    /// record.
    fn scale_viewport_ext(
        self,
        _record_number: usize,
        _record: META_SCALEVIEWPORTEXT,
    ) -> Result<Self, PlayError> {
        info!("META_SCALEVIEWPORTEXT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SCALEWINDOWEXT`](crate::parser::META_SCALEWINDOWEXT)
    /// record.
    fn scale_window_ext(
        self,
        _record_number: usize,
        _record: META_SCALEWINDOWEXT,
    ) -> Result<Self, PlayError> {
        info!("META_SCALEWINDOWEXT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETBKCOLOR`](crate::parser::META_SETBKCOLOR) record.
    fn set_bk_color(
        self,
        _record_number: usize,
        _record: META_SETBKCOLOR,
    ) -> Result<Self, PlayError> {
        info!("META_SETBKCOLOR: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETBKMODE`](crate::parser::META_SETBKMODE) record.
    fn set_bk_mode(
        self,
        _record_number: usize,
        _record: META_SETBKMODE,
    ) -> Result<Self, PlayError> {
        info!("META_SETBKMODE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETLAYOUT`](crate::parser::META_SETLAYOUT) record.
    fn set_layout(
        self,
        _record_number: usize,
        _record: META_SETLAYOUT,
    ) -> Result<Self, PlayError> {
        info!("META_SETLAYOUT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETMAPMODE`](crate::parser::META_SETMAPMODE) record.
    fn set_map_mode(
        self,
        _record_number: usize,
        _record: META_SETMAPMODE,
    ) -> Result<Self, PlayError> {
        info!("META_SETMAPMODE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETMAPPERFLAGS`](crate::parser::META_SETMAPPERFLAGS)
    /// record.
    fn set_mapper_flags(
        self,
        _record_number: usize,
        _record: META_SETMAPPERFLAGS,
    ) -> Result<Self, PlayError> {
        info!("META_SETMAPPERFLAGS: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETPALENTRIES`](crate::parser::META_SETPALENTRIES) record.
    fn set_pal_entries(
        self,
        _record_number: usize,
        _record: META_SETPALENTRIES,
    ) -> Result<Self, PlayError> {
        info!("META_SETPALENTRIES: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETPOLYFILLMODE`](crate::parser::META_SETPOLYFILLMODE)
    /// record.
    fn set_polyfill_mode(
        self,
        _record_number: usize,
        _record: META_SETPOLYFILLMODE,
    ) -> Result<Self, PlayError> {
        info!("META_SETPOLYFILLMODE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETRELABS`](crate::parser::META_SETRELABS) record.
    fn set_relabs(
        self,
        _record_number: usize,
        _record: META_SETRELABS,
    ) -> Result<Self, PlayError> {
        info!("META_SETRELABS: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETROP2`](crate::parser::META_SETROP2) record.
    fn set_raster_operation(
        self,
        _record_number: usize,
        _record: META_SETROP2,
    ) -> Result<Self, PlayError> {
        info!("META_SETROP2: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETSTRETCHBLTMODE`](crate::parser::META_SETSTRETCHBLTMODE)
    /// record.
    fn set_stretch_blt_mode(
        self,
        _record_number: usize,
        _record: META_SETSTRETCHBLTMODE,
    ) -> Result<Self, PlayError> {
        info!("META_SETSTRETCHBLTMODE: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETTEXTALIGN`](crate::parser::META_SETTEXTALIGN) record.
    fn set_text_align(
        self,
        _record_number: usize,
        _record: META_SETTEXTALIGN,
    ) -> Result<Self, PlayError> {
        info!("META_SETTEXTALIGN: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETTEXTCHAREXTRA`](crate::parser::META_SETTEXTCHAREXTRA)
    /// record.
    fn set_text_char_extra(
        self,
        _record_number: usize,
        _record: META_SETTEXTCHAREXTRA,
    ) -> Result<Self, PlayError> {
        info!("META_SETTEXTCHAREXTRA: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETTEXTCOLOR`](crate::parser::META_SETTEXTCOLOR) record.
    fn set_text_color(
        self,
        _record_number: usize,
        _record: META_SETTEXTCOLOR,
    ) -> Result<Self, PlayError> {
        info!("META_SETTEXTCOLOR: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETTEXTJUSTIFICATION`](crate::parser::META_SETTEXTJUSTIFICATION) record.
    fn set_text_justification(
        self,
        _record_number: usize,
        _record: META_SETTEXTJUSTIFICATION,
    ) -> Result<Self, PlayError> {
        info!("META_SETTEXTJUSTIFICATION: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETVIEWPORTEXT`](crate::parser::META_SETVIEWPORTEXT)
    /// record.
    fn set_viewport_ext(
        self,
        _record_number: usize,
        _record: META_SETVIEWPORTEXT,
    ) -> Result<Self, PlayError> {
        info!("META_SETVIEWPORTEXT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETVIEWPORTORG`](crate::parser::META_SETVIEWPORTORG)
    /// record.
    fn set_viewport_origin(
        self,
        _record_number: usize,
        _record: META_SETVIEWPORTORG,
    ) -> Result<Self, PlayError> {
        info!("META_SETVIEWPORTORG: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETWINDOWEXT`](crate::parser::META_SETWINDOWEXT) record.
    fn set_window_ext(
        self,
        _record_number: usize,
        _record: META_SETWINDOWEXT,
    ) -> Result<Self, PlayError> {
        info!("META_SETWINDOWEXT: skipped (not implemented)");
        Ok(self)
    }
    /// Render [`META_SETWINDOWORG`](crate::parser::META_SETWINDOWORG) record.
    fn set_window_origin(
        self,
        _record_number: usize,
        _record: META_SETWINDOWORG,
    ) -> Result<Self, PlayError> {
        info!("META_SETWINDOWORG: skipped (not implemented)");
        Ok(self)
    }

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .

    /// Render [`META_ESCAPE`](crate::parser::META_ESCAPE) record.
    fn escape(
        self,
        _record_number: usize,
        _record: META_ESCAPE,
    ) -> Result<Self, PlayError> {
        info!("META_ESCAPE: skipped (not implemented)");
        Ok(self)
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
