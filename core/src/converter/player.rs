use crate::parser::*;

#[derive(Clone, Debug, thiserror::Error)]
pub enum PlayError {
    #[error("failed to generate: {cause}")]
    FailedGenerate { cause: String },
    #[error("invalid brush: {cause}")]
    InvalidBrush { cause: String },
    #[error("invalid record: {cause}")]
    InvalidRecord { cause: String },
    #[error("unexpected graphics object: {cause}")]
    UnexpectedGraphicsObject { cause: String },
    #[error("unknown: {cause}")]
    Unknown { cause: String },
}

pub trait Player {
    /// Call after converting to write output.
    fn generate(self) -> Result<(), PlayError>;

    // .
    // .
    // Functions to support parsing Records
    // .
    // .
    fn selected_font(&self) -> Result<&Font, PlayError>;

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    fn bit_blt(&mut self, record: META_BITBLT) -> Result<(), PlayError>;
    fn device_independent_bitmap_bit_blt(
        &mut self,
        record: META_DIBBITBLT,
    ) -> Result<(), PlayError>;
    fn device_independent_bitmap_stretch_blt(
        &mut self,
        record: META_DIBSTRETCHBLT,
    ) -> Result<(), PlayError>;
    fn set_device_independent_bitmap_to_dev(
        &mut self,
        record: META_SETDIBTODEV,
    ) -> Result<(), PlayError>;
    fn stretch_blt(&mut self, record: META_STRETCHBLT)
        -> Result<(), PlayError>;
    fn stretch_device_independent_bitmap(
        &mut self,
        record: META_STRETCHDIB,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    fn eof(&mut self, record: META_EOF) -> Result<(), PlayError>;
    fn header(&mut self, header: MetafileHeader) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .
    fn arc(&mut self, record: META_ARC) -> Result<(), PlayError>;
    fn chord(&mut self, record: META_CHORD) -> Result<(), PlayError>;
    fn ellipse(&mut self, record: META_ELLIPSE) -> Result<(), PlayError>;
    fn ext_flood_fill(
        &mut self,
        record: META_EXTFLOODFILL,
    ) -> Result<(), PlayError>;
    fn ext_text_out(
        &mut self,
        record: META_EXTTEXTOUT,
    ) -> Result<(), PlayError>;
    fn fill_region(&mut self, record: META_FILLREGION)
        -> Result<(), PlayError>;
    fn flood_fill(&mut self, record: META_FLOODFILL) -> Result<(), PlayError>;
    fn frame_region(
        &mut self,
        record: META_FRAMEREGION,
    ) -> Result<(), PlayError>;
    fn invert_region(
        &mut self,
        record: META_INVERTREGION,
    ) -> Result<(), PlayError>;
    fn line_to(&mut self, record: META_LINETO) -> Result<(), PlayError>;
    fn paint_region(
        &mut self,
        record: META_PAINTREGION,
    ) -> Result<(), PlayError>;
    fn pat_blt(&mut self, record: META_PATBLT) -> Result<(), PlayError>;
    fn pie(&mut self, record: META_PIE) -> Result<(), PlayError>;
    fn polyline(&mut self, record: META_POLYLINE) -> Result<(), PlayError>;
    fn polygon(&mut self, record: META_POLYGON) -> Result<(), PlayError>;
    fn poly_polygon(
        &mut self,
        record: META_POLYPOLYGON,
    ) -> Result<(), PlayError>;
    fn reactangle(&mut self, record: META_RECTANGLE) -> Result<(), PlayError>;
    fn round_rect(&mut self, record: META_ROUNDRECT) -> Result<(), PlayError>;
    fn set_pixel(&mut self, record: META_SETPIXEL) -> Result<(), PlayError>;
    fn text_out(
        &mut self,
        record: META_TEXTOUT,
    ) -> Result<(), crate::converter::PlayError>;

    // .
    // .
    // Functions to handle Object Record
    // .
    // .
    fn create_brush_indirect(
        &mut self,
        record: META_CREATEBRUSHINDIRECT,
    ) -> Result<(), PlayError>;
    fn create_font_indirect(
        &mut self,
        record: META_CREATEFONTINDIRECT,
    ) -> Result<(), PlayError>;
    fn create_palette(
        &mut self,
        record: META_CREATEPALETTE,
    ) -> Result<(), PlayError>;
    fn create_pattern_brush(
        &mut self,
        record: META_CREATEPATTERNBRUSH,
    ) -> Result<(), PlayError>;
    fn create_pen_indirect(
        &mut self,
        record: META_CREATEPENINDIRECT,
    ) -> Result<(), PlayError>;
    fn create_region(
        &mut self,
        record: META_CREATEREGION,
    ) -> Result<(), crate::converter::PlayError>;
    fn delete_object(
        &mut self,
        record: META_DELETEOBJECT,
    ) -> Result<(), PlayError>;
    fn create_device_independent_bitmap_pattern_brush(
        &mut self,
        record: META_DIBCREATEPATTERNBRUSH,
    ) -> Result<(), PlayError>;
    fn select_clip_region(
        &mut self,
        record: META_SELECTCLIPREGION,
    ) -> Result<(), PlayError>;
    fn select_object(
        &mut self,
        record: META_SELECTOBJECT,
    ) -> Result<(), PlayError>;
    fn select_palette(
        &mut self,
        record: META_SELECTPALETTE,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    fn animate_palette(
        &mut self,
        record: META_ANIMATEPALETTE,
    ) -> Result<(), PlayError>;
    fn exclude_clip_rect(
        &mut self,
        record: META_EXCLUDECLIPRECT,
    ) -> Result<(), PlayError>;
    fn intersect_clip_rect(
        &mut self,
        record: META_INTERSECTCLIPRECT,
    ) -> Result<(), PlayError>;
    fn move_to(&mut self, record: META_MOVETO) -> Result<(), PlayError>;
    fn offset_clip_region(
        &mut self,
        record: META_OFFSETCLIPRGN,
    ) -> Result<(), PlayError>;
    fn offset_viewport_origin(
        &mut self,
        record: META_OFFSETVIEWPORTORG,
    ) -> Result<(), PlayError>;
    fn offset_window_origin(
        &mut self,
        record: META_OFFSETWINDOWORG,
    ) -> Result<(), PlayError>;
    fn realize_palette(
        &mut self,
        record: META_REALIZEPALETTE,
    ) -> Result<(), PlayError>;
    fn resize_palette(
        &mut self,
        record: META_RESIZEPALETTE,
    ) -> Result<(), PlayError>;
    fn restore_device_context(
        &mut self,
        record: META_RESTOREDC,
    ) -> Result<(), PlayError>;
    fn save_device_context(
        &mut self,
        record: META_SAVEDC,
    ) -> Result<(), PlayError>;
    fn scale_viewport_ext(
        &mut self,
        record: META_SCALEVIEWPORTEXT,
    ) -> Result<(), PlayError>;
    fn scale_window_ext(
        &mut self,
        record: META_SCALEWINDOWEXT,
    ) -> Result<(), PlayError>;
    fn set_bk_color(
        &mut self,
        record: META_SETBKCOLOR,
    ) -> Result<(), PlayError>;
    fn set_bk_mode(&mut self, record: META_SETBKMODE) -> Result<(), PlayError>;
    fn set_layout(&mut self, record: META_SETLAYOUT) -> Result<(), PlayError>;
    fn set_map_mode(
        &mut self,
        record: META_SETMAPMODE,
    ) -> Result<(), PlayError>;
    fn set_mapper_flags(
        &mut self,
        record: META_SETMAPPERFLAGS,
    ) -> Result<(), PlayError>;
    fn set_pal_entries(
        &mut self,
        record: META_SETPALENTRIES,
    ) -> Result<(), PlayError>;
    fn set_polyfill_mode(
        &mut self,
        record: META_SETPOLYFILLMODE,
    ) -> Result<(), PlayError>;
    fn set_relabs(&mut self, record: META_SETRELABS) -> Result<(), PlayError>;
    fn set_raster_operation(
        &mut self,
        record: META_SETROP2,
    ) -> Result<(), PlayError>;
    fn set_stretch_blt_mode(
        &mut self,
        record: META_SETSTRETCHBLTMODE,
    ) -> Result<(), PlayError>;
    fn set_text_align(
        &mut self,
        record: META_SETTEXTALIGN,
    ) -> Result<(), PlayError>;
    fn set_text_char_extra(
        &mut self,
        record: META_SETTEXTCHAREXTRA,
    ) -> Result<(), PlayError>;
    fn set_text_color(
        &mut self,
        record: META_SETTEXTCOLOR,
    ) -> Result<(), PlayError>;
    fn set_text_justification(
        &mut self,
        record: META_SETTEXTJUSTIFICATION,
    ) -> Result<(), PlayError>;
    fn set_viewport_ext(
        &mut self,
        record: META_SETVIEWPORTEXT,
    ) -> Result<(), PlayError>;
    fn set_viewport_origin(
        &mut self,
        record: META_SETVIEWPORTORG,
    ) -> Result<(), PlayError>;
    fn set_window_ext(
        &mut self,
        record: META_SETWINDOWEXT,
    ) -> Result<(), PlayError>;
    fn set_window_origin(
        &mut self,
        record: META_SETWINDOWORG,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .
    fn escape(&mut self, record: META_ESCAPE) -> Result<(), PlayError>;
}
