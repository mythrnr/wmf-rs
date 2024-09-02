mod device_context;
mod graphics_object;

use std::collections::{BTreeSet, VecDeque};

use svg::{
    node::element::{path::Data, Image, Path, Polygon, Rectangle, Text},
    Document,
};
use wmf_core::*;

use self::{device_context::*, graphics_object::*};

pub struct SVGPlayer<W> {
    context_stack: Vec<DeviceContext>,
    context_current: DeviceContext,
    document: Document,
    objects: crate::GraphicsObjects,
    object_selected: crate::SelectedGraphicsObject,
    output: W,
}

impl<W> SVGPlayer<W> {
    pub fn new(output: W) -> Self {
        Self {
            context_stack: Vec::with_capacity(0),
            context_current: Default::default(),
            document: Document::new(),
            objects: crate::GraphicsObjects::new(0),
            object_selected: crate::SelectedGraphicsObject::default(),
            output,
        }
    }

    #[inline]
    fn current_context(&self) -> DeviceContext {
        self.context_current.clone()
    }

    #[inline]
    fn set_current_context(&mut self, context: DeviceContext) {
        self.context_current = context.into();
    }

    fn selected_brush(&self) -> Result<Brush, crate::PlayError> {
        let Some(ref brush) = self.object_selected.brush else {
            return Err(crate::PlayError::UnexpectedGraphicsObject {
                cause: format!("{:?}", self.object_selected),
            });
        };

        Ok(brush.clone())
    }

    fn selected_pen(&self) -> Result<Pen, crate::PlayError> {
        let Some(ref pen) = self.object_selected.pen else {
            return Err(crate::PlayError::UnexpectedGraphicsObject {
                cause: format!("{:?}", self.object_selected),
            });
        };

        Ok(pen.clone())
    }
}

impl<W> crate::Player for SVGPlayer<W>
where
    W: std::io::Write,
{
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn generate(&mut self) -> Result<(), crate::PlayError> {
        self.output.write(&self.document.to_string().into_bytes()).map_err(
            |err| crate::PlayError::FailedGenerate { cause: err.to_string() },
        )?;

        Ok(())
    }

    // .
    // .
    // Functions to support parsing Records
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn selected_font(&self) -> Result<Font, crate::PlayError> {
        let Some(ref font) = self.object_selected.font else {
            return Err(crate::PlayError::UnexpectedGraphicsObject {
                cause: format!("{:?}", self.object_selected),
            });
        };

        Ok(font.clone())
    }

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn bit_blt(
        &mut self,
        _record: META_BITBLT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_BITBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn device_independent_bitmap_bit_blt(
        &mut self,
        _record: META_DIBBITBLT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_DIBBITBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn device_independent_bitmap_stretch_blt(
        &mut self,
        _record: META_DIBSTRETCHBLT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_DIBSTRETCHBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_device_independent_bitmap_to_dev(
        &mut self,
        _record: META_SETDIBTODEV,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETDIBTODEV: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn stretch_blt(
        &mut self,
        _record: META_STRETCHBLT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_STRETCHBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn stretch_device_independent_bitmap(
        &mut self,
        record: META_STRETCHDIB,
    ) -> Result<(), crate::PlayError> {
        let data = crate::Bitmap::from(record.dib).as_data_url();

        let image = Image::new()
            .set("x", record.x_dst)
            .set("y", record.y_dst)
            .set("width", record.dest_width)
            .set("height", record.dest_height)
            .set("src", data);

        self.document = self.document.clone().add(image);

        Ok(())
    }

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn eof(&mut self, _: META_EOF) -> Result<(), crate::PlayError> {
        self.document = self
            .document
            .clone()
            .set("viewBox", self.current_context().window.as_view_box());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn header(
        &mut self,
        header: MetafileHeader,
    ) -> Result<(), crate::PlayError> {
        let (_placeable, header) = match header {
            MetafileHeader::StartsWithHeader(header) => (None, header),
            MetafileHeader::StartsWithPlaceable(placeable, header) => {
                (Some(placeable), header)
            }
        };

        self.objects =
            crate::GraphicsObjects::new(header.number_of_objects as usize);

        Ok(())
    }

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn arc(&mut self, _record: META_ARC) -> Result<(), crate::PlayError> {
        tracing::info!("META_ARC: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn chord(&mut self, _record: META_CHORD) -> Result<(), crate::PlayError> {
        tracing::info!("META_CHORD: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn ellipse(
        &mut self,
        _record: META_ELLIPSE,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_ELLIPSE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn ext_flood_fill(
        &mut self,
        _record: META_EXTFLOODFILL,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_EXTFLOODFILL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn ext_text_out(
        &mut self,
        record: META_EXTTEXTOUT,
    ) -> Result<(), crate::PlayError> {
        let context = self.current_context();
        let font = self.selected_font()?;
        let decoration = {
            let mut v = vec![];

            if font.underline {
                v.push("underline");
            }

            if font.strike_out {
                v.push("line-through");
            }

            v
        };
        let point = PointS { x: record.x, y: record.y };
        let update_cp = context
            .text_align
            .contains(&(TextAlignmentMode::TA_UPDATECP as u16));
        let c = if update_cp {
            context.point_s_to_relative_point(&point)
        } else {
            context.point_s_to_absolute_point(&point)
        };
        let text_align = context.text_horizon_align();
        let text = Text::new(record.string)
            .set("x", c.x)
            .set("y", c.y)
            .set("text-anchor", text_align)
            .set("fill", context.text_color_as_css_color())
            .set("font-family", font.facename)
            .set("font-size", font.height.abs())
            .set("font-weight", font.weight)
            .set("rotate", font.orientation)
            .set(
                "style",
                format!(
                    "{}{}",
                    if decoration.is_empty() {
                        "".to_owned()
                    } else {
                        format!("text-decoration: {};", decoration.join(" "))
                    },
                    if font.italic {
                        "font-style: italic;"
                    } else {
                        "font-style: normal;"
                    },
                ),
            );

        if update_cp {
            self.set_current_context(context.current_coordinate(c));
        }

        self.document = self.document.clone().add(text);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn fill_region(
        &mut self,
        _record: META_FILLREGION,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_FILLREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn flood_fill(
        &mut self,
        _record: META_FLOODFILL,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_FLOODFILL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn frame_region(
        &mut self,
        _record: META_FRAMEREGION,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_FRAMEREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn invert_region(
        &mut self,
        _record: META_INVERTREGION,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_INVERTREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn line_to(&mut self, record: META_LINETO) -> Result<(), crate::PlayError> {
        let context = self.current_context();
        let stroke = Stroke::from(self.selected_pen()?);
        let point = PointS { x: record.x, y: record.y };
        let c = context.point_s_to_absolute_point(&point);

        let data = Data::new()
            .move_to((
                context.current_coordinate.x,
                context.current_coordinate.y,
            ))
            .line_to((c.x, c.y));
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("d", data);

        self.set_current_context(context.current_coordinate(c));
        self.document = self.document.clone().add(path);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn paint_region(
        &mut self,
        _record: META_PAINTREGION,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_PAINTREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn pat_blt(&mut self, record: META_PATBLT) -> Result<(), crate::PlayError> {
        if record.width == 0 || record.height == 0 {
            tracing::info!(
                %record.width,
                %record.height,
                "META_PATBLT is skipped because width or height is zero.",
            );

            return Ok(());
        }

        let mut document = self.document.clone();
        let context = self.current_context();
        let pen = self.selected_pen()?;
        let stroke = Stroke::from(pen);
        let brush = self.selected_brush()?;
        let fill = match Fill::from(brush) {
            Fill::Pattern { mut pattern } => {
                let id = random_string();

                pattern = pattern.set("id", id.clone());
                document = document.add(pattern);

                url_string(format!("#{id}"))
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();

        let rect = Rectangle::new()
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("x", record.x_left)
            .set("y", record.y_left)
            .set("height", record.height)
            .set("width", record.width);

        self.document = document.add(rect);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn pie(&mut self, _record: META_PIE) -> Result<(), crate::PlayError> {
        tracing::info!("META_PIE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn polyline(
        &mut self,
        record: META_POLYLINE,
    ) -> Result<(), crate::PlayError> {
        let context = self.current_context();
        let stroke = Stroke::from(self.selected_pen()?);

        let Some(point) = record.a_points.get(0) else {
            return Err(crate::PlayError::InvalidRecord {
                cause: format!("aPoints[0] is not defined"),
            });
        };

        let mut coordinate = context.point_s_to_absolute_point(&point);
        let mut data = Data::new().move_to((coordinate.x, coordinate.y));

        for i in 1..record.number_of_points {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(crate::PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };
            coordinate = context.point_s_to_absolute_point(&point);

            data = data.line_to((coordinate.x, coordinate.y));
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("d", data);

        self.set_current_context(context.current_coordinate(coordinate));
        self.document = self.document.clone().add(path);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn polygon(
        &mut self,
        record: META_POLYGON,
    ) -> Result<(), crate::PlayError> {
        let mut document = self.document.clone();
        let context = self.current_context();
        let pen = self.selected_pen()?;
        let stroke = Stroke::from(pen);
        let brush = self.selected_brush()?;
        let fill = match Fill::from(brush) {
            Fill::Pattern { mut pattern } => {
                let id = random_string();

                pattern = pattern.set("id", id.clone());
                document = document.add(pattern);

                url_string(format!("#{id}"))
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();

        let mut points = vec![];

        for i in 0..record.number_of_points {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(crate::PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            let c = context.point_s_to_absolute_point(point);
            points.push(c.as_point_string());
        }

        let polygon = Polygon::new()
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("points", points.join(" "));

        self.document = document.clone().add(polygon);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn poly_polygon(
        &mut self,
        record: META_POLYPOLYGON,
    ) -> Result<(), crate::PlayError> {
        let mut document = self.document.clone();
        let context = self.current_context();
        let pen = self.selected_pen()?;
        let stroke = Stroke::from(pen);
        let brush = self.selected_brush()?;
        let fill = match Fill::from(brush) {
            Fill::Pattern { mut pattern } => {
                let id = random_string();

                pattern = pattern.set("id", id.clone());
                document = document.add(pattern);

                url_string(format!("#{id}"))
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();

        let mut a_point: VecDeque<_> = record.poly_polygon.a_points.into();
        let mut current_point_index = 0;

        for i in 0..record.poly_polygon.number_of_polygons {
            let Some(points_of_polygon) =
                record.poly_polygon.a_points_per_polygon.get(i as usize)
            else {
                return Err(crate::PlayError::InvalidRecord {
                    cause: format!("aPointsPerPolygon[{i}] is not defined"),
                });
            };

            let mut points = vec![];

            for _ in 0..*points_of_polygon {
                let Some(point) = a_point.pop_front() else {
                    return Err(crate::PlayError::InvalidRecord {
                        cause: format!(
                            "aPoints[{current_point_index}] is not defined"
                        ),
                    });
                };
                let c = context.point_s_to_absolute_point(&point);

                points.push(c.as_point_string());
                current_point_index += 1;
            }

            let polygon = Polygon::new()
                .set("fill", fill.as_str())
                .set("fill-rule", fill_rule.as_str())
                .set("stroke", stroke.color())
                .set("stroke-width", stroke.width())
                .set("stroke-opacity", stroke.opacity())
                .set("stroke-linecap", stroke.line_cap())
                .set("stroke-dasharray", stroke.dash_array())
                .set("stroke-linejoin", stroke.line_join())
                .set("points", points.join(" "));

            document = document.clone().add(polygon);
        }

        self.document = document;

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn reactangle(
        &mut self,
        record: META_RECTANGLE,
    ) -> Result<(), crate::PlayError> {
        let mut document = self.document.clone();
        let context = self.current_context();
        let pen = self.selected_pen()?;
        let stroke = Stroke::from(pen);
        let brush = self.selected_brush()?;
        let fill = match Fill::from(brush) {
            Fill::Pattern { mut pattern } => {
                let id = random_string();

                pattern = pattern.set("id", id.clone());
                document = document.add(pattern);

                url_string(format!("#{id}"))
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let rect = Rectangle::new()
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("x", record.left_rect)
            .set("y", record.top_rect)
            .set("height", record.right_rect - record.left_rect)
            .set("width", record.bottom_rect - record.top_rect);

        self.document = document.add(rect);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn round_rect(
        &mut self,
        _record: META_ROUNDRECT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_ROUNDRECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_pixel(
        &mut self,
        _record: META_SETPIXEL,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETPIXEL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn text_out(
        &mut self,
        record: META_TEXTOUT,
    ) -> Result<(), crate::PlayError> {
        let context = self.current_context();
        let font = self.selected_font()?;
        let decoration = {
            let mut v = vec![];

            if font.underline {
                v.push("underline");
            }

            if font.strike_out {
                v.push("line-through");
            }

            v
        };

        let point = PointS { x: record.x_start, y: record.y_start };
        let c = context.point_s_to_relative_point(&point);
        let text = Text::new(record.string)
            .set("x", c.x)
            .set("y", font.height.abs() + c.y)
            .set("fill", context.text_color_as_css_color())
            .set("font-family", font.facename)
            .set("font-size", font.height.abs())
            .set("font-weight", font.weight)
            .set("rotate", font.orientation)
            .set(
                "style",
                format!(
                    "{}{}",
                    if decoration.is_empty() {
                        "".to_owned()
                    } else {
                        format!("text-decoration: {};", decoration.join(" "))
                    },
                    if font.italic {
                        "font-style: italic;"
                    } else {
                        "font-style: normal;"
                    },
                ),
            );

        self.document = self.document.clone().add(text);

        Ok(())
    }

    // .
    // .
    // Functions to handle Object Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn create_brush_indirect(
        &mut self,
        record: META_CREATEBRUSHINDIRECT,
    ) -> Result<(), crate::PlayError> {
        self.objects.push(crate::GraphicsObject::Brush(record.create_brush()));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn create_font_indirect(
        &mut self,
        record: META_CREATEFONTINDIRECT,
    ) -> Result<(), crate::PlayError> {
        self.objects.push(crate::GraphicsObject::Font(record.font));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn create_palette(
        &mut self,
        record: META_CREATEPALETTE,
    ) -> Result<(), crate::PlayError> {
        self.objects.push(crate::GraphicsObject::Palette(record.palette));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn create_pattern_brush(
        &mut self,
        _record: META_CREATEPATTERNBRUSH,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_CREATEPATTERNBRUSH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn create_pen_indirect(
        &mut self,
        record: META_CREATEPENINDIRECT,
    ) -> Result<(), crate::PlayError> {
        self.objects.push(crate::GraphicsObject::Pen(record.pen));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn create_region(
        &mut self,
        record: META_CREATEREGION,
    ) -> Result<(), crate::PlayError> {
        self.objects.push(crate::GraphicsObject::Region(record.region));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn delete_object(
        &mut self,
        record: META_DELETEOBJECT,
    ) -> Result<(), crate::PlayError> {
        self.objects.delete(record.object_index as usize);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn create_device_independent_bitmap_pattern_brush(
        &mut self,
        record: META_DIBCREATEPATTERNBRUSH,
    ) -> Result<(), crate::PlayError> {
        self.objects.push(crate::GraphicsObject::Brush(record.create_brush()));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn select_clip_region(
        &mut self,
        _record: META_SELECTCLIPREGION,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SELECTCLIPREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn select_object(
        &mut self,
        record: META_SELECTOBJECT,
    ) -> Result<(), crate::PlayError> {
        let object = self.objects.get(record.object_index as usize).clone();
        let selected = self.object_selected.clone();

        self.object_selected = match object {
            crate::GraphicsObject::Brush(v) => selected.brush(v),
            crate::GraphicsObject::Font(v) => selected.font(v),
            crate::GraphicsObject::Palette(v) => selected.palette(v),
            crate::GraphicsObject::Pen(v) => selected.pen(v),
            crate::GraphicsObject::Region(v) => selected.region(v),
            _ => {
                return Err(crate::PlayError::UnexpectedGraphicsObject {
                    cause: "Graphics Object is null".to_owned(),
                })
            }
        };

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn select_palette(
        &mut self,
        _record: META_SELECTPALETTE,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SELECTPALETTE: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn animate_palette(
        &mut self,
        _record: META_ANIMATEPALETTE,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_ANIMATEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn exclude_clip_rect(
        &mut self,
        _record: META_EXCLUDECLIPRECT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_EXCLUDECLIPRECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn intersect_clip_rect(
        &mut self,
        record: META_INTERSECTCLIPRECT,
    ) -> Result<(), crate::PlayError> {
        let META_INTERSECTCLIPRECT { bottom, right, top, left, .. } = record;
        self.set_current_context(
            self.current_context().clipping_region(Rect {
                left,
                top,
                right,
                bottom,
            }),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn move_to(&mut self, record: META_MOVETO) -> Result<(), crate::PlayError> {
        let context = self.current_context();
        // let point = PointS { x: record.x, y: record.y };
        // let c = context.point_s_to_absolute_point(&point);
        let c = Coordinate { x: record.x, y: record.y };

        self.set_current_context(context.current_coordinate(c));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn offset_clip_region(
        &mut self,
        _record: META_OFFSETCLIPRGN,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_OFFSETCLIPRGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn offset_viewport_origin(
        &mut self,
        _record: META_OFFSETVIEWPORTORG,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_OFFSETVIEWPORTORG: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn offset_window_origin(
        &mut self,
        _record: META_OFFSETWINDOWORG,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_OFFSETWINDOWORG: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn realize_palette(
        &mut self,
        _record: META_REALIZEPALETTE,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_REALIZEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn resize_palette(
        &mut self,
        _record: META_RESIZEPALETTE,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_RESIZEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn restore_device_context(
        &mut self,
        _record: META_RESTOREDC,
    ) -> Result<(), crate::PlayError> {
        let context = self.context_stack.pop().unwrap_or_default();
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn save_device_context(
        &mut self,
        _record: META_SAVEDC,
    ) -> Result<(), crate::PlayError> {
        self.context_stack.push(self.current_context());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn scale_viewport_ext(
        &mut self,
        _record: META_SCALEVIEWPORTEXT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SCALEVIEWPORTEXT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn scale_window_ext(
        &mut self,
        record: META_SCALEWINDOWEXT,
    ) -> Result<(), crate::PlayError> {
        let context = self.current_context();
        let scale_x = (context.window.scale_x * record.x_num as f32)
            / record.x_denom as f32;
        let scale_y = (context.window.scale_y * record.y_num as f32)
            / record.y_denom as f32;

        self.set_current_context(context.window_scale(scale_x, scale_y));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_bk_color(
        &mut self,
        record: META_SETBKCOLOR,
    ) -> Result<(), crate::PlayError> {
        self.set_current_context(
            self.current_context().bk_color(record.color_ref),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_bk_mode(
        &mut self,
        record: META_SETBKMODE,
    ) -> Result<(), crate::PlayError> {
        self.set_current_context(
            self.current_context().bk_mode(record.bk_mode),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_layout(
        &mut self,
        _record: META_SETLAYOUT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETLAYOUT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_map_mode(
        &mut self,
        record: META_SETMAPMODE,
    ) -> Result<(), crate::PlayError> {
        self.set_current_context(
            self.current_context().map_mode(record.map_mode),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_mapper_flags(
        &mut self,
        _record: META_SETMAPPERFLAGS,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETMAPPERFLAGS: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_pal_entries(
        &mut self,
        _record: META_SETPALENTRIES,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETPALENTRIES: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_polyfill_mode(
        &mut self,
        record: META_SETPOLYFILLMODE,
    ) -> Result<(), crate::PlayError> {
        self.set_current_context(
            self.current_context().poly_fill_mode(record.poly_fill_mode),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_relabs(
        &mut self,
        _record: META_SETRELABS,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETRELABS: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_raster_operation(
        &mut self,
        record: META_SETROP2,
    ) -> Result<(), crate::PlayError> {
        self.set_current_context(
            self.current_context().draw_mode(record.draw_mode),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_stretch_blt_mode(
        &mut self,
        _record: META_SETSTRETCHBLTMODE,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETSTRETCHBLTMODE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_text_align(
        &mut self,
        record: META_SETTEXTALIGN,
    ) -> Result<(), crate::PlayError> {
        use strum::IntoEnumIterator;

        let mut align = BTreeSet::new();
        let mut aligns = TextAlignmentMode::iter();
        let mut v_aligns = VerticalTextAlignmentMode::iter();

        while let Some(v) = aligns.next() {
            if record.text_alignment_mode & (v as u16) == v as u16 {
                align.insert(v as u16);
            }
        }

        while let Some(v) = v_aligns.next() {
            if record.text_alignment_mode & (v as u16) == v as u16 {
                align.insert(v as u16);
            }
        }

        if align.is_empty() {
            align.insert(0x0000);
        }

        self.set_current_context(self.current_context().text_align(align));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_text_char_extra(
        &mut self,
        _record: META_SETTEXTCHAREXTRA,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETTEXTCHAREXTRA: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_text_color(
        &mut self,
        record: META_SETTEXTCOLOR,
    ) -> Result<(), crate::PlayError> {
        self.set_current_context(
            self.current_context().text_color(record.color_ref),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_text_justification(
        &mut self,
        _record: META_SETTEXTJUSTIFICATION,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETTEXTJUSTIFICATION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_viewport_ext(
        &mut self,
        _record: META_SETVIEWPORTEXT,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETVIEWPORTEXT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_viewport_origin(
        &mut self,
        _record: META_SETVIEWPORTORG,
    ) -> Result<(), crate::PlayError> {
        tracing::info!("META_SETVIEWPORTORG: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_window_ext(
        &mut self,
        record: META_SETWINDOWEXT,
    ) -> Result<(), crate::PlayError> {
        let context = self.current_context().window_ext(record.x, record.y);

        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn set_window_origin(
        &mut self,
        record: META_SETWINDOWORG,
    ) -> Result<(), crate::PlayError> {
        let context = self.current_context().window_origin(record.x, record.y);

        self.set_current_context(context);

        Ok(())
    }

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display),
    )]
    fn escape(&mut self, _record: META_ESCAPE) -> Result<(), crate::PlayError> {
        Ok(())
    }
}
