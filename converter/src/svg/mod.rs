mod device_context;
mod util;

use std::collections::VecDeque;

use svg::{
    node::element::{
        path::Data, Ellipse, Image, Path, Polygon, Rectangle, Text,
    },
    Document,
};
use wmf_core::*;

use self::{device_context::*, util::*};

pub struct SVGPlayer<W> {
    context_stack: Vec<DeviceContext>,
    context_current: DeviceContext,
    document: Document,
    object_selected: crate::SelectedGraphicsObject,
    output: W,
}

impl<W> SVGPlayer<W> {
    pub fn new(output: W) -> Self {
        Self {
            context_stack: Vec::with_capacity(0),
            context_current: Default::default(),
            document: Document::new(),
            object_selected: crate::SelectedGraphicsObject::default(),
            output,
        }
    }

    #[inline]
    fn current_context(&self) -> &DeviceContext {
        &self.context_current
    }

    #[inline]
    fn set_current_context(&mut self, context: DeviceContext) {
        self.context_current = context;
    }

    fn selected_brush(&self) -> Result<&Brush, crate::PlayError> {
        Ok(&self.object_selected.brush)
    }

    fn selected_pen(&self) -> Result<&Pen, crate::PlayError> {
        Ok(&self.object_selected.pen)
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
    fn selected_font(&self) -> Result<&Font, crate::PlayError> {
        Ok(&self.object_selected.font)
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
        let context = self.current_context().clone();
        let (_placeable, header) = match header {
            MetafileHeader::StartsWithHeader(header) => (None, header),
            MetafileHeader::StartsWithPlaceable(placeable, header) => {
                (Some(placeable), header)
            }
        };

        self.set_current_context(
            context.create_object_table(header.number_of_objects),
        );

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
        tracing::info!("META_CHORD: not implemented");
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
        record: META_ELLIPSE,
    ) -> Result<(), crate::PlayError> {
        let (rx, ry) = (
            (record.right_rect - record.left_rect) / 2,
            (record.bottom_rect - record.top_rect) / 2,
        );

        if rx == 0 || ry == 0 {
            tracing::info!(
                %rx, %ry,
                "META_ELLIPSE is skipped because rx or ry is zero.",
            );

            return Ok(());
        }

        let mut document = self.document.clone();
        let context = self.current_context();
        let stroke = Stroke::from(self.selected_pen()?.clone());
        let fill = match Fill::from(self.selected_brush()?.clone()) {
            Fill::Pattern { mut pattern } => {
                let id = random_string();

                pattern = pattern.set("id", id.clone());
                document = document.add(pattern);

                url_string(format!("#{id}"))
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let point = context.point_s_to_absolute_point(&PointS {
            x: record.left_rect + rx,
            y: record.top_rect + ry,
        });

        let ellipse = Ellipse::new()
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("cx", point.x)
            .set("cy", point.y)
            .set("rx", rx)
            .set("ry", ry);

        self.document = document.add(ellipse);

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
        let point = PointS {
            x: record.x,
            y: record.y
                + (if context.text_align_vertical
                    == VerticalTextAlignmentMode::VTA_TOP
                    && font.height > 0
                {
                    font.height
                } else if matches!(
                    context.text_align_vertical,
                    VerticalTextAlignmentMode::VTA_BASELINE
                        | VerticalTextAlignmentMode::VTA_BOTTOM
                ) && font.height < 0
                {
                    -1 * font.height
                } else {
                    0
                }),
        };
        let c = if context.text_align_update_cp {
            context.point_s_to_relative_point(&point)
        } else {
            context.point_s_to_absolute_point(&point)
        };
        let text_align = context.as_css_text_align();
        let shape_inside = if let (true, Some(rect)) = (
            record.fw_opts.contains(&ExtTextOutOptions::ETO_CLIPPED),
            record.rectangle,
        ) {
            let tl = {
                let point = PointS { x: rect.left, y: rect.top };
                if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                }
            };
            let tr = {
                let point = PointS { x: rect.right, y: rect.top };
                if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                }
            };
            let bl = {
                let point = PointS { x: rect.left, y: rect.bottom };
                if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                }
            };
            let br = {
                let point = PointS { x: rect.right, y: rect.bottom };
                if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                }
            };

            format!(
                "shape-inside: polygon({},{} {},{} {},{} {},{});",
                tl.x, tl.y, bl.x, bl.y, tr.x, tr.y, br.x, br.y,
            )
        } else {
            "".to_owned()
        };
        let text = Text::new(record.string)
            .set("x", c.x)
            .set("y", c.y)
            .set("text-anchor", text_align)
            .set("fill", context.text_color_as_css_color())
            .set("font-family", font.facename.clone())
            .set("font-size", font.height.abs())
            .set("font-weight", font.weight)
            .set("rotate", font.orientation)
            .set(
                "style",
                format!(
                    "{}{}{}",
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
                    shape_inside,
                ),
            );

        if context.text_align_update_cp {
            self.set_current_context(context.clone().drawing_position(c));
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
        let stroke = Stroke::from(self.selected_pen()?.clone());
        let point = PointS { x: record.x, y: record.y };
        let c = context.point_s_to_absolute_point(&point);

        let data = Data::new()
            .move_to((context.drawing_position.x, context.drawing_position.y))
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

        self.set_current_context(context.clone().drawing_position(c));
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
        let brush = self.selected_brush()?;
        let stroke = Stroke::from(brush.clone());
        let fill = match Fill::from(brush.clone()) {
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
        let stroke = Stroke::from(self.selected_pen()?.clone());

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

        self.set_current_context(context.clone().drawing_position(coordinate));
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
        let stroke = Stroke::from(self.selected_pen()?.clone());
        let fill = match Fill::from(self.selected_brush()?.clone()) {
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

            let p = context.point_s_to_absolute_point(point);
            points.push(as_point_string(&p));
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
        let stroke = Stroke::from(self.selected_pen()?.clone());
        let fill = match Fill::from(self.selected_brush()?.clone()) {
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
                let p = context.point_s_to_absolute_point(&point);

                points.push(as_point_string(&p));
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
        let stroke = Stroke::from(self.selected_pen()?.clone());
        let fill = match Fill::from(self.selected_brush()?.clone()) {
            Fill::Pattern { mut pattern } => {
                let id = random_string();

                pattern = pattern.set("id", id.clone());
                document = document.add(pattern);

                url_string(format!("#{id}"))
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let tl = {
            let point = PointS { x: record.left_rect, y: record.top_rect };
            context.point_s_to_absolute_point(&point)
        };
        let br = {
            let point = PointS { x: record.right_rect, y: record.bottom_rect };
            context.point_s_to_absolute_point(&point)
        };

        let rect = Rectangle::new()
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("x", tl.x)
            .set("y", tl.y)
            .set("height", br.y - tl.y)
            .set("width", br.x - tl.x);

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
        record: META_ROUNDRECT,
    ) -> Result<(), crate::PlayError> {
        let (width, height) = (
            record.right_rect - record.left_rect,
            record.bottom_rect - record.top_rect,
        );

        if width == 0 || height == 0 {
            tracing::info!(
                %width, %height,
                "META_ROUNDRECT is skipped because width or height is zero.",
            );

            return Ok(());
        }

        let mut document = self.document.clone();
        let context = self.current_context();
        let stroke = Stroke::from(self.selected_pen()?.clone());
        let fill = match Fill::from(self.selected_brush()?.clone()) {
            Fill::Pattern { mut pattern } => {
                let id = random_string();

                pattern = pattern.set("id", id.clone());
                document = document.add(pattern);

                url_string(format!("#{id}"))
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let point = context.point_s_to_absolute_point(&PointS {
            x: record.left_rect,
            y: record.top_rect,
        });

        let rect = Rectangle::new()
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", stroke.color())
            .set("stroke-width", stroke.width())
            .set("stroke-opacity", stroke.opacity())
            .set("stroke-linecap", stroke.line_cap())
            .set("stroke-dasharray", stroke.dash_array())
            .set("stroke-linejoin", stroke.line_join())
            .set("x", point.x)
            .set("y", point.y)
            .set("height", height)
            .set("width", width)
            .set("rx", record.width)
            .set("ry", record.height);

        self.document = document.add(rect);

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
            .set("font-family", font.facename.clone())
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
        let mut context = self.current_context().clone();

        context
            .object_table
            .push(crate::GraphicsObject::Brush(record.create_brush()));
        self.set_current_context(context);

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
        let mut context = self.current_context().clone();

        context.object_table.push(crate::GraphicsObject::Font(record.font));
        self.set_current_context(context);

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
        let mut context = self.current_context().clone();

        context
            .object_table
            .push(crate::GraphicsObject::Palette(record.palette));
        self.set_current_context(context);

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
        // let mut context = self.current_context().clone();

        // context
        //     .object_table
        //     .push(crate::GraphicsObject::Brush(record.create_brush()));
        // self.set_current_context(context);

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
        let mut context = self.current_context().clone();

        context.object_table.push(crate::GraphicsObject::Pen(record.pen));
        self.set_current_context(context);

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
        let mut context = self.current_context().clone();

        context.object_table.push(crate::GraphicsObject::Region(record.region));
        self.set_current_context(context);

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
        let mut context = self.current_context().clone();

        context.object_table.delete(record.object_index as usize);
        self.set_current_context(context);

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
        let mut context = self.current_context().clone();

        context
            .object_table
            .push(crate::GraphicsObject::Brush(record.create_brush()));
        self.set_current_context(context);

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
        let object = self
            .current_context()
            .object_table
            .get(record.object_index as usize)
            .clone();
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
            self.current_context().clone().clipping_region(Rect {
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
        // TODO:
        // let point = PointS { x: record.x, y: record.y };
        // let c = context.point_s_to_absolute_point(&point);
        let p = PointS { x: record.x, y: record.y };

        self.set_current_context(
            self.current_context().clone().drawing_position(p),
        );

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
        record: META_RESTOREDC,
    ) -> Result<(), crate::PlayError> {
        let context = if record.n_saved_dc < 0 {
            self.current_context().clone().into()
        } else if (record.n_saved_dc as usize) < self.context_stack.len() {
            self.context_stack.remove(record.n_saved_dc as usize).into()
        } else {
            None
        };

        self.set_current_context(
            context.unwrap_or_else(|| self.current_context().clone()),
        );

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
        self.context_stack.push(self.current_context().clone());

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

        self.set_current_context(
            context.clone().window_scale(scale_x, scale_y),
        );

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
            self.current_context().clone().text_bk_color(record.color_ref),
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
            self.current_context().clone().bk_mode(record.bk_mode),
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
            self.current_context().clone().map_mode(record.map_mode),
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
            self.current_context()
                .clone()
                .poly_fill_mode(record.poly_fill_mode),
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
            self.current_context().clone().draw_mode(record.draw_mode),
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
        let update_cp = record.text_alignment_mode
            & (TextAlignmentMode::TA_UPDATECP as u16)
            == TextAlignmentMode::TA_UPDATECP as u16;
        let align_horizontal =
            [TextAlignmentMode::TA_CENTER, TextAlignmentMode::TA_RIGHT]
                .into_iter()
                .find(|a| record.text_alignment_mode & (*a as u16) == *a as u16)
                .unwrap_or_else(|| TextAlignmentMode::TA_LEFT);
        let align_vertical = [
            VerticalTextAlignmentMode::VTA_BOTTOM,
            VerticalTextAlignmentMode::VTA_TOP,
        ]
        .into_iter()
        .find(|a| record.text_alignment_mode & (*a as u16) == *a as u16)
        .unwrap_or_else(|| VerticalTextAlignmentMode::VTA_BASELINE);

        let context = self
            .current_context()
            .clone()
            .text_align_update_cp(update_cp)
            .text_align_horizontal(align_horizontal)
            .text_align_vertical(align_vertical);

        self.set_current_context(context);

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
            self.current_context().clone().text_color(record.color_ref),
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
        self.set_current_context(
            self.current_context().clone().window_ext(record.x, record.y),
        );

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
        self.set_current_context(
            self.current_context().clone().window_origin(record.x, record.y),
        );

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
