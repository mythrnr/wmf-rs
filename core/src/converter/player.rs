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

pub trait Player: Sized {
    /// Call after converting to write output.
    fn generate(self) -> Result<Vec<u8>, PlayError>;

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
    fn bit_blt(self, record: META_BITBLT) -> Result<Self, PlayError>;
    fn device_independent_bitmap_bit_blt(
        self,
        record: META_DIBBITBLT,
    ) -> Result<Self, PlayError>;
    fn device_independent_bitmap_stretch_blt(
        self,
        record: META_DIBSTRETCHBLT,
    ) -> Result<Self, PlayError>;
    fn set_device_independent_bitmap_to_dev(
        self,
        record: META_SETDIBTODEV,
    ) -> Result<Self, PlayError>;
    fn stretch_blt(self, record: META_STRETCHBLT) -> Result<Self, PlayError>;
    fn stretch_device_independent_bitmap(
        self,
        record: META_STRETCHDIB,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    fn eof(self, record: META_EOF) -> Result<Self, PlayError>;
    fn header(self, header: MetafileHeader) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .
    fn arc(self, record: META_ARC) -> Result<Self, PlayError>;
    fn chord(self, record: META_CHORD) -> Result<Self, PlayError>;
    fn ellipse(self, record: META_ELLIPSE) -> Result<Self, PlayError>;
    fn ext_flood_fill(
        self,
        record: META_EXTFLOODFILL,
    ) -> Result<Self, PlayError>;
    fn ext_text_out(self, record: META_EXTTEXTOUT) -> Result<Self, PlayError>;
    fn fill_region(self, record: META_FILLREGION) -> Result<Self, PlayError>;
    fn flood_fill(self, record: META_FLOODFILL) -> Result<Self, PlayError>;
    fn frame_region(self, record: META_FRAMEREGION) -> Result<Self, PlayError>;
    fn invert_region(
        self,
        record: META_INVERTREGION,
    ) -> Result<Self, PlayError>;
    fn line_to(self, record: META_LINETO) -> Result<Self, PlayError>;
    fn paint_region(self, record: META_PAINTREGION) -> Result<Self, PlayError>;
    fn pat_blt(self, record: META_PATBLT) -> Result<Self, PlayError>;
    fn pie(self, record: META_PIE) -> Result<Self, PlayError>;
    fn polyline(self, record: META_POLYLINE) -> Result<Self, PlayError>;
    fn polygon(self, record: META_POLYGON) -> Result<Self, PlayError>;
    fn poly_polygon(self, record: META_POLYPOLYGON) -> Result<Self, PlayError>;
    fn rectangle(self, record: META_RECTANGLE) -> Result<Self, PlayError>;
    fn round_rect(self, record: META_ROUNDRECT) -> Result<Self, PlayError>;
    fn set_pixel(self, record: META_SETPIXEL) -> Result<Self, PlayError>;
    fn text_out(
        self,
        record: META_TEXTOUT,
    ) -> Result<Self, crate::converter::PlayError>;

    // .
    // .
    // Functions to handle Object Record
    // .
    // .
    fn create_brush_indirect(
        self,
        record: META_CREATEBRUSHINDIRECT,
    ) -> Result<Self, PlayError>;
    fn create_font_indirect(
        self,
        record: META_CREATEFONTINDIRECT,
    ) -> Result<Self, PlayError>;
    fn create_palette(
        self,
        record: META_CREATEPALETTE,
    ) -> Result<Self, PlayError>;
    fn create_pattern_brush(
        self,
        record: META_CREATEPATTERNBRUSH,
    ) -> Result<Self, PlayError>;
    fn create_pen_indirect(
        self,
        record: META_CREATEPENINDIRECT,
    ) -> Result<Self, PlayError>;
    fn create_region(
        self,
        record: META_CREATEREGION,
    ) -> Result<Self, crate::converter::PlayError>;
    fn delete_object(
        self,
        record: META_DELETEOBJECT,
    ) -> Result<Self, PlayError>;
    fn create_device_independent_bitmap_pattern_brush(
        self,
        record: META_DIBCREATEPATTERNBRUSH,
    ) -> Result<Self, PlayError>;
    fn select_clip_region(
        self,
        record: META_SELECTCLIPREGION,
    ) -> Result<Self, PlayError>;
    fn select_object(
        self,
        record: META_SELECTOBJECT,
    ) -> Result<Self, PlayError>;
    fn select_palette(
        self,
        record: META_SELECTPALETTE,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    fn animate_palette(
        self,
        record: META_ANIMATEPALETTE,
    ) -> Result<Self, PlayError>;
    fn exclude_clip_rect(
        self,
        record: META_EXCLUDECLIPRECT,
    ) -> Result<Self, PlayError>;
    fn intersect_clip_rect(
        self,
        record: META_INTERSECTCLIPRECT,
    ) -> Result<Self, PlayError>;
    fn move_to(self, record: META_MOVETO) -> Result<Self, PlayError>;
    fn offset_clip_region(
        self,
        record: META_OFFSETCLIPRGN,
    ) -> Result<Self, PlayError>;
    fn offset_viewport_origin(
        self,
        record: META_OFFSETVIEWPORTORG,
    ) -> Result<Self, PlayError>;
    fn offset_window_origin(
        self,
        record: META_OFFSETWINDOWORG,
    ) -> Result<Self, PlayError>;
    fn realize_palette(
        self,
        record: META_REALIZEPALETTE,
    ) -> Result<Self, PlayError>;
    fn resize_palette(
        self,
        record: META_RESIZEPALETTE,
    ) -> Result<Self, PlayError>;
    fn restore_device_context(
        self,
        record: META_RESTOREDC,
    ) -> Result<Self, PlayError>;
    fn save_device_context(
        self,
        record: META_SAVEDC,
    ) -> Result<Self, PlayError>;
    fn scale_viewport_ext(
        self,
        record: META_SCALEVIEWPORTEXT,
    ) -> Result<Self, PlayError>;
    fn scale_window_ext(
        self,
        record: META_SCALEWINDOWEXT,
    ) -> Result<Self, PlayError>;
    fn set_bk_color(self, record: META_SETBKCOLOR) -> Result<Self, PlayError>;
    fn set_bk_mode(self, record: META_SETBKMODE) -> Result<Self, PlayError>;
    fn set_layout(self, record: META_SETLAYOUT) -> Result<Self, PlayError>;
    fn set_map_mode(self, record: META_SETMAPMODE) -> Result<Self, PlayError>;
    fn set_mapper_flags(
        self,
        record: META_SETMAPPERFLAGS,
    ) -> Result<Self, PlayError>;
    fn set_pal_entries(
        self,
        record: META_SETPALENTRIES,
    ) -> Result<Self, PlayError>;
    fn set_polyfill_mode(
        self,
        record: META_SETPOLYFILLMODE,
    ) -> Result<Self, PlayError>;
    fn set_relabs(self, record: META_SETRELABS) -> Result<Self, PlayError>;
    fn set_raster_operation(
        self,
        record: META_SETROP2,
    ) -> Result<Self, PlayError>;
    fn set_stretch_blt_mode(
        self,
        record: META_SETSTRETCHBLTMODE,
    ) -> Result<Self, PlayError>;
    fn set_text_align(
        self,
        record: META_SETTEXTALIGN,
    ) -> Result<Self, PlayError>;
    fn set_text_char_extra(
        self,
        record: META_SETTEXTCHAREXTRA,
    ) -> Result<Self, PlayError>;
    fn set_text_color(
        self,
        record: META_SETTEXTCOLOR,
    ) -> Result<Self, PlayError>;
    fn set_text_justification(
        self,
        record: META_SETTEXTJUSTIFICATION,
    ) -> Result<Self, PlayError>;
    fn set_viewport_ext(
        self,
        record: META_SETVIEWPORTEXT,
    ) -> Result<Self, PlayError>;
    fn set_viewport_origin(
        self,
        record: META_SETVIEWPORTORG,
    ) -> Result<Self, PlayError>;
    fn set_window_ext(
        self,
        record: META_SETWINDOWEXT,
    ) -> Result<Self, PlayError>;
    fn set_window_origin(
        self,
        record: META_SETWINDOWORG,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .
    fn escape(self, record: META_ESCAPE) -> Result<Self, PlayError>;
}
