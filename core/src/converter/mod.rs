mod bitmap;
mod graphics_object;
mod player;

pub use self::player::*;
use self::{bitmap::*, graphics_object::*};
use crate::parser::*;

#[cfg(feature = "svg")]
mod svg;

#[cfg(feature = "svg")]
pub use self::svg::*;

#[derive(Clone, Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("parse error: {source}")]
    ParseError { source: ParseError },
    #[error("play error: {source}")]
    PlayError { source: crate::converter::PlayError },
}

impl From<ParseError> for ConvertError {
    fn from(source: ParseError) -> Self {
        Self::ParseError { source }
    }
}

impl From<crate::converter::PlayError> for ConvertError {
    fn from(source: crate::converter::PlayError) -> Self {
        Self::PlayError { source }
    }
}

pub struct WMFConverter<B, P> {
    buffer: B,
    player: P,
}

impl<B, P> WMFConverter<B, P> {
    pub fn new(buffer: B, player: P) -> Self {
        Self { buffer, player }
    }
}

impl<B, P> WMFConverter<B, P>
where
    B: std::io::Read,
    P: crate::converter::Player,
{
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    pub fn run(self) -> Result<(), ConvertError> {
        let Self { mut buffer, mut player } = self;
        let buf = &mut buffer;

        let (header, _) = MetafileHeader::parse(buf)?;

        tracing::debug!(?header);
        player.header(header)?;

        let mut record_number = 0;

        loop {
            record_number += 1;
            tracing::debug!(%record_number);

            let mut record_size = RecordSize::parse(buf)?;
            if record_size.byte_count() == 0 {
                tracing::debug!(%record_size, "skip parsing zero-sized record");

                continue;
            }

            let (record_function, c) =
                read_u16_from_le_bytes(buf).map_err(ParseError::from)?;
            record_size.consume(c);

            tracing::debug!(
                %record_size,
                record_function = %format!("{record_function:#06X}"),
            );

            let Some(record_type) = RecordType::from_repr(record_function)
            else {
                tracing::debug!(
                    record_function = %format!("{record_function:#06X}"),
                    "record_function is not match any RecordType",
                );

                return Err(ConvertError::ParseError {
                    source: ParseError::UnexpectedEnumValue {
                        cause: format!(
                            "record_function `{record_function:#06X}` is not \
                             match any RecordType"
                        ),
                    },
                });
            };

            match record_type {
                // bitmap record
                RecordType::META_BITBLT => {
                    let record =
                        META_BITBLT::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.bit_blt(record)?;
                }
                RecordType::META_DIBBITBLT => {
                    let record = META_DIBBITBLT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.device_independent_bitmap_bit_blt(record)?;
                }
                RecordType::META_DIBSTRETCHBLT => {
                    let record = META_DIBSTRETCHBLT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.device_independent_bitmap_stretch_blt(record)?;
                }
                RecordType::META_SETDIBTODEV => {
                    let record = META_SETDIBTODEV::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_device_independent_bitmap_to_dev(record)?;
                }
                RecordType::META_STRETCHBLT => {
                    let record = META_STRETCHBLT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.stretch_blt(record)?;
                }
                RecordType::META_STRETCHDIB => {
                    let record = META_STRETCHDIB::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.stretch_device_independent_bitmap(record)?;
                }
                // control record
                RecordType::META_EOF => {
                    let record =
                        META_EOF::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.eof(record)?;
                    break;
                }
                // drawing record
                RecordType::META_ARC => {
                    let record =
                        META_ARC::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.arc(record)?;
                }
                RecordType::META_CHORD => {
                    let record =
                        META_CHORD::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.chord(record)?;
                }
                RecordType::META_ELLIPSE => {
                    let record =
                        META_ELLIPSE::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.ellipse(record)?;
                }
                RecordType::META_EXTFLOODFILL => {
                    let record = META_EXTFLOODFILL::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.ext_flood_fill(record)?;
                }
                RecordType::META_EXTTEXTOUT => {
                    let font = player.selected_font()?;
                    let record = META_EXTTEXTOUT::parse(
                        buf,
                        record_size,
                        record_function,
                        font.charset,
                    )?;

                    tracing::debug!(?record);
                    player.ext_text_out(record)?;
                }
                RecordType::META_FILLREGION => {
                    let record = META_FILLREGION::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.fill_region(record)?;
                }
                RecordType::META_FLOODFILL => {
                    let record = META_FLOODFILL::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.flood_fill(record)?;
                }
                RecordType::META_FRAMEREGION => {
                    let record = META_FRAMEREGION::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.frame_region(record)?;
                }
                RecordType::META_INVERTREGION => {
                    let record = META_INVERTREGION::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.invert_region(record)?;
                }
                RecordType::META_LINETO => {
                    let record =
                        META_LINETO::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.line_to(record)?;
                }
                RecordType::META_PAINTREGION => {
                    let record = META_PAINTREGION::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.paint_region(record)?;
                }
                RecordType::META_PATBLT => {
                    let record =
                        META_PATBLT::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.pat_blt(record)?;
                }
                RecordType::META_PIE => {
                    let record =
                        META_PIE::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.pie(record)?;
                }
                RecordType::META_POLYLINE => {
                    let record = META_POLYLINE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.polyline(record)?;
                }
                RecordType::META_POLYGON => {
                    let record =
                        META_POLYGON::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.polygon(record)?;
                }
                RecordType::META_POLYPOLYGON => {
                    let record = META_POLYPOLYGON::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.poly_polygon(record)?;
                }
                RecordType::META_RECTANGLE => {
                    let record = META_RECTANGLE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.reactangle(record)?;
                }
                RecordType::META_ROUNDRECT => {
                    let record = META_ROUNDRECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.round_rect(record)?;
                }
                RecordType::META_SETPIXEL => {
                    let record = META_SETPIXEL::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_pixel(record)?;
                }
                RecordType::META_TEXTOUT => {
                    let font = player.selected_font()?;
                    let record = META_TEXTOUT::parse(
                        buf,
                        record_size,
                        record_function,
                        font.charset,
                    )?;

                    tracing::debug!(?record);
                    player.text_out(record)?;
                }
                // object record
                RecordType::META_CREATEBRUSHINDIRECT => {
                    let record = META_CREATEBRUSHINDIRECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.create_brush_indirect(record)?;
                }
                RecordType::META_CREATEFONTINDIRECT => {
                    let record = META_CREATEFONTINDIRECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.create_font_indirect(record)?;
                }
                RecordType::META_CREATEPALETTE => {
                    let record = META_CREATEPALETTE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.create_palette(record)?;
                }
                RecordType::META_CREATEPATTERNBRUSH => {
                    let record = META_CREATEPATTERNBRUSH::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.create_pattern_brush(record)?;
                }
                RecordType::META_CREATEPENINDIRECT => {
                    let record = META_CREATEPENINDIRECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.create_pen_indirect(record)?;
                }
                RecordType::META_CREATEREGION => {
                    let record = META_CREATEREGION::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.create_region(record)?;
                }
                RecordType::META_DELETEOBJECT => {
                    let record = META_DELETEOBJECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.delete_object(record)?;
                }
                RecordType::META_DIBCREATEPATTERNBRUSH => {
                    let record = META_DIBCREATEPATTERNBRUSH::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.create_device_independent_bitmap_pattern_brush(
                        record,
                    )?;
                }
                RecordType::META_SELECTCLIPREGION => {
                    let record = META_SELECTCLIPREGION::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.select_clip_region(record)?;
                }
                RecordType::META_SELECTOBJECT => {
                    let record = META_SELECTOBJECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.select_object(record)?;
                }
                RecordType::META_SELECTPALETTE => {
                    let record = META_SELECTPALETTE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.select_palette(record)?;
                }
                // state record
                RecordType::META_ANIMATEPALETTE => {
                    let record = META_ANIMATEPALETTE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.animate_palette(record)?;
                }
                RecordType::META_EXCLUDECLIPRECT => {
                    let record = META_EXCLUDECLIPRECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.exclude_clip_rect(record)?;
                }
                RecordType::META_INTERSECTCLIPRECT => {
                    let record = META_INTERSECTCLIPRECT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.intersect_clip_rect(record)?;
                }
                RecordType::META_MOVETO => {
                    let record =
                        META_MOVETO::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.move_to(record)?;
                }
                RecordType::META_OFFSETCLIPRGN => {
                    let record = META_OFFSETCLIPRGN::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.offset_clip_region(record)?;
                }
                RecordType::META_OFFSETVIEWPORTORG => {
                    let record = META_OFFSETVIEWPORTORG::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.offset_viewport_origin(record)?;
                }
                RecordType::META_OFFSETWINDOWORG => {
                    let record = META_OFFSETWINDOWORG::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.offset_window_origin(record)?;
                }
                RecordType::META_REALIZEPALETTE => {
                    let record = META_REALIZEPALETTE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.realize_palette(record)?;
                }
                RecordType::META_RESIZEPALETTE => {
                    let record = META_RESIZEPALETTE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.resize_palette(record)?;
                }
                RecordType::META_RESTOREDC => {
                    let record = META_RESTOREDC::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.restore_device_context(record)?;
                }
                RecordType::META_SAVEDC => {
                    let record =
                        META_SAVEDC::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.save_device_context(record)?;
                }
                RecordType::META_SCALEVIEWPORTEXT => {
                    let record = META_SCALEVIEWPORTEXT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.scale_viewport_ext(record)?;
                }
                RecordType::META_SCALEWINDOWEXT => {
                    let record = META_SCALEWINDOWEXT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.scale_window_ext(record)?;
                }
                RecordType::META_SETBKCOLOR => {
                    let record = META_SETBKCOLOR::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_bk_color(record)?;
                }
                RecordType::META_SETBKMODE => {
                    let record = META_SETBKMODE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_bk_mode(record)?;
                }
                RecordType::META_SETLAYOUT => {
                    let record = META_SETLAYOUT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_layout(record)?;
                }
                RecordType::META_SETMAPMODE => {
                    let record = META_SETMAPMODE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_map_mode(record)?;
                }
                RecordType::META_SETMAPPERFLAGS => {
                    let record = META_SETMAPPERFLAGS::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_mapper_flags(record)?;
                }
                RecordType::META_SETPALENTRIES => {
                    let record = META_SETPALENTRIES::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_pal_entries(record)?;
                }
                RecordType::META_SETPOLYFILLMODE => {
                    let record = META_SETPOLYFILLMODE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_polyfill_mode(record)?;
                }
                RecordType::META_SETRELABS => {
                    let record = META_SETRELABS::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_relabs(record)?;
                }
                RecordType::META_SETROP2 => {
                    let record =
                        META_SETROP2::parse(buf, record_size, record_function)?;

                    tracing::debug!(?record);
                    player.set_raster_operation(record)?;
                }
                RecordType::META_SETSTRETCHBLTMODE => {
                    let record = META_SETSTRETCHBLTMODE::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_stretch_blt_mode(record)?;
                }
                RecordType::META_SETTEXTALIGN => {
                    let record = META_SETTEXTALIGN::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_text_align(record)?;
                }
                RecordType::META_SETTEXTCHAREXTRA => {
                    let record = META_SETTEXTCHAREXTRA::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_text_char_extra(record)?;
                }
                RecordType::META_SETTEXTCOLOR => {
                    let record = META_SETTEXTCOLOR::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_text_color(record)?;
                }
                RecordType::META_SETTEXTJUSTIFICATION => {
                    let record = META_SETTEXTJUSTIFICATION::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_text_justification(record)?;
                }
                RecordType::META_SETVIEWPORTEXT => {
                    let record = META_SETVIEWPORTEXT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_viewport_ext(record)?;
                }
                RecordType::META_SETVIEWPORTORG => {
                    let record = META_SETVIEWPORTORG::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_viewport_origin(record)?;
                }
                RecordType::META_SETWINDOWEXT => {
                    let record = META_SETWINDOWEXT::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_window_ext(record)?;
                }
                RecordType::META_SETWINDOWORG => {
                    let record = META_SETWINDOWORG::parse(
                        buf,
                        record_size,
                        record_function,
                    )?;

                    tracing::debug!(?record);
                    player.set_window_origin(record)?;
                }
                // escape record
                RecordType::META_ESCAPE => {
                    let r = read_variable(buf, record_size.remaining_bytes());
                    tracing::debug!(?r, "META_ESCAPE skipped");
                    // let record =
                    //     META_ESCAPE::parse(buf, record_size,
                    // record_function)?;

                    // tracing::debug!(?record);
                    // player.escape(record)?;
                }
            };
        }

        player.generate()?;

        Ok(())
    }
}
