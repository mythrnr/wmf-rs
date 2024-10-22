mod device_context;
mod node;
mod ternary_raster_operator;
mod util;

use crate::{
    converter::{
        svg::{
            device_context::DeviceContext,
            node::{Data, Node},
            ternary_raster_operator::TernaryRasterOperator,
            util::{as_point_string, url_string, Fill, Stroke},
        },
        GraphicsObject, PlayError, SelectedGraphicsObject,
    },
    imports::*,
    parser::*,
};

pub struct SVGPlayer {
    context_stack: Vec<DeviceContext>,
    context_current: DeviceContext,
    definitions: Vec<Node>,
    elements: Vec<Node>,
    object_selected: SelectedGraphicsObject,
}

impl SVGPlayer {
    pub fn new() -> Self {
        Self {
            context_stack: Vec::with_capacity(0),
            context_current: DeviceContext::default(),
            definitions: vec![],
            elements: vec![],
            object_selected: SelectedGraphicsObject::default(),
        }
    }

    #[inline]
    fn current_context(&self) -> &DeviceContext {
        &self.context_current
    }

    #[inline]
    fn issue_id(&self) -> String {
        format!("defs{}", self.definitions.len())
    }

    #[inline]
    fn set_current_context(&mut self, context: DeviceContext) {
        self.context_current = context;
    }

    fn selected_brush(&self) -> &Brush {
        &self.object_selected.brush
    }

    fn selected_pen(&self) -> &Pen {
        &self.object_selected.pen
    }
}

impl crate::converter::Player for SVGPlayer {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        let Self { context_current, definitions, elements, .. } = self;

        let (x, y, width, height) = context_current.window.as_view_box();
        let mut document = Node::node("svg")
            .set("xmlns", "http://www.w3.org/2000/svg")
            .set("viewBox", format!("{x} {y} {width} {height}"));

        if !definitions.is_empty() {
            let mut defs = Node::node("defs");
            for v in definitions {
                defs = defs.add(v);
            }

            document = document.add(defs);
        }

        for v in elements {
            document = document.add(v);
        }

        Ok(document.to_string().into_bytes())
    }

    // .
    // .
    // Functions to support parsing Records
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn selected_font(&self) -> Result<&Font, PlayError> {
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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn bit_blt(&mut self, record: META_BITBLT) -> Result<(), PlayError> {
        let operator = match record {
            META_BITBLT::WithBitmap {
                raster_operation,
                height,
                width,
                y_dest,
                x_dest,
                target,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    height,
                    width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                if raster_operation.use_source() {
                    operator = operator.source_bitmap16(target);
                }

                operator
            }
            META_BITBLT::WithoutBitmap {
                raster_operation,
                height,
                width,
                y_dest,
                x_dest,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    height,
                    width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                operator
            }
        };

        let Some(elem) =
            operator.run(&mut self.definitions).map_err(|err| {
                PlayError::InvalidRecord { cause: err.to_string() }
            })?
        else {
            return Ok(());
        };

        self.elements.push(elem);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn device_independent_bitmap_bit_blt(
        &mut self,
        record: META_DIBBITBLT,
    ) -> Result<(), PlayError> {
        let operator = match record {
            META_DIBBITBLT::WithBitmap {
                raster_operation,
                height,
                width,
                y_dest,
                x_dest,
                target,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    height,
                    width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                if raster_operation.use_source() {
                    operator = operator.source_bitmap(*target);
                }

                operator
            }
            META_DIBBITBLT::WithoutBitmap {
                raster_operation,
                height,
                width,
                y_dest,
                x_dest,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    height,
                    width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                operator
            }
        };

        let Some(elem) =
            operator.run(&mut self.definitions).map_err(|err| {
                PlayError::InvalidRecord { cause: err.to_string() }
            })?
        else {
            return Ok(());
        };

        self.elements.push(elem);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn device_independent_bitmap_stretch_blt(
        &mut self,
        record: META_DIBSTRETCHBLT,
    ) -> Result<(), PlayError> {
        let operator = match record {
            META_DIBSTRETCHBLT::WithBitmap {
                raster_operation,
                dest_height,
                dest_width,
                y_dest,
                x_dest,
                target,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    dest_height,
                    dest_width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                if raster_operation.use_source() {
                    operator = operator.source_bitmap(*target);
                }

                operator
            }
            META_DIBSTRETCHBLT::WithoutBitmap {
                raster_operation,
                dest_height,
                dest_width,
                y_dest,
                x_dest,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    dest_height,
                    dest_width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                operator
            }
        };

        let Some(elem) =
            operator.run(&mut self.definitions).map_err(|err| {
                PlayError::InvalidRecord { cause: err.to_string() }
            })?
        else {
            return Ok(());
        };

        self.elements.push(elem);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_device_independent_bitmap_to_dev(
        &mut self,
        _record: META_SETDIBTODEV,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETDIBTODEV: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn stretch_blt(
        &mut self,
        record: META_STRETCHBLT,
    ) -> Result<(), PlayError> {
        let operator = match record {
            META_STRETCHBLT::WithBitmap {
                raster_operation,
                dest_height,
                dest_width,
                y_dest,
                x_dest,
                target,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    dest_height,
                    dest_width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                if raster_operation.use_source() {
                    operator = operator.source_bitmap16(target);
                }

                operator
            }
            META_STRETCHBLT::WithoutBitmap {
                raster_operation,
                dest_height,
                dest_width,
                y_dest,
                x_dest,
                ..
            } => {
                let mut operator = TernaryRasterOperator::new(
                    raster_operation,
                    x_dest,
                    y_dest,
                    dest_height,
                    dest_width,
                );

                if raster_operation.use_selected_brush() {
                    operator = operator.brush(self.selected_brush().clone());
                }

                operator
            }
        };

        let Some(elem) =
            operator.run(&mut self.definitions).map_err(|err| {
                PlayError::InvalidRecord { cause: err.to_string() }
            })?
        else {
            return Ok(());
        };

        self.elements.push(elem);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn stretch_device_independent_bitmap(
        &mut self,
        record: META_STRETCHDIB,
    ) -> Result<(), PlayError> {
        let META_STRETCHDIB {
            raster_operation,
            dest_height,
            dest_width,
            y_dst,
            x_dst,
            dib,
            ..
        } = record;

        let mut operator = TernaryRasterOperator::new(
            raster_operation,
            x_dst,
            y_dst,
            dest_height,
            dest_width,
        );

        if raster_operation.use_selected_brush() {
            operator = operator.brush(self.selected_brush().clone());
        }

        if raster_operation.use_source() {
            operator = operator.source_bitmap(dib);
        }

        let Some(elem) =
            operator.run(&mut self.definitions).map_err(|err| {
                PlayError::InvalidRecord { cause: err.to_string() }
            })?
        else {
            return Ok(());
        };

        self.elements.push(elem);

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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn eof(&mut self, _: META_EOF) -> Result<(), PlayError> {
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn header(&mut self, header: MetafileHeader) -> Result<(), PlayError> {
        let (_placeable, header) = match header {
            MetafileHeader::StartsWithHeader(header) => (None, header),
            MetafileHeader::StartsWithPlaceable(placeable, header) => {
                (Some(placeable), header)
            }
        };

        self.set_current_context(
            self.current_context()
                .clone()
                .create_object_table(header.number_of_objects),
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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn arc(&mut self, record: META_ARC) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());
        let start = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.x_start_arc,
                y: record.y_start_arc,
            });

            context = context.extend_window(&point);
            point
        };
        let end = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.x_end_arc,
                y: record.y_end_arc,
            });

            context = context.extend_window(&point);
            point
        };
        let (rx, ry) = (
            (record.right_rect - record.left_rect) / 2,
            (record.bottom_rect - record.top_rect) / 2,
        );
        let center = context.point_s_to_absolute_point(&PointS {
            x: record.left_rect + rx,
            y: record.top_rect + ry,
        });

        let center_x = if center.x < start.x && center.x < end.x {
            "left"
        } else if start.x < center.x && end.x < center.x {
            "right"
        } else {
            "middle"
        };
        let center_y = if start.y < center.y && end.y < center.y {
            "over"
        } else if center.y < start.y && center.y < end.y {
            "under"
        } else {
            "middle"
        };
        // if center_x or center_y is middle,
        // fix sweep as negative-angle direction.
        #[allow(clippy::match_same_arms)]
        let (large_arc, sweep) = match (center_x, center_y) {
            ("left", "over") if start.x < end.x && start.y > end.y => (0, 0),
            ("left", "over") if start.x > end.x && start.y < end.y => (0, 1),
            ("left", "middle") if start.x < end.x && start.y < end.y => (1, 0),
            ("left", "middle") if start.x < end.x && start.y > end.y => (0, 0),
            ("left", "middle") if start.x > end.x && start.y < end.y => (1, 0),
            ("left", "middle") if start.x > end.x && start.y > end.y => (0, 0),
            ("left", "under") if start.x < end.x && start.y < end.y => (0, 1),
            ("left", "under") if start.x > end.x && start.y > end.y => (0, 0),
            ("right", "over") if start.x < end.x && start.y < end.y => (0, 0),
            ("right", "over") if start.x > end.x && start.y > end.y => (0, 1),
            ("right", "middle") if start.x < end.x && start.y < end.y => (0, 0),
            ("right", "middle") if start.x < end.x && start.y > end.y => (1, 0),
            ("right", "middle") if start.x > end.x && start.y < end.y => (0, 0),
            ("right", "middle") if start.x > end.x && start.y > end.y => (1, 0),
            ("right", "under") if start.x < end.x && start.y > end.y => (0, 1),
            ("right", "under") if start.x > end.x && start.y < end.y => (0, 0),
            ("middle", "over") if start.x < end.x && start.y < end.y => (0, 0),
            ("middle", "over") if start.x > end.x && start.y > end.y => (1, 0),
            ("middle", "under") if start.x < end.x && start.y < end.y => (1, 0),
            ("middle", "under") if start.x > end.x && start.y > end.y => (0, 0),
            ("middle", "middle") if start.x <= end.x && start.y <= end.y => {
                let antipodal = PointS {
                    x: start.x + (center.x - start.x) * 2,
                    y: start.y + (center.y - start.y) * 2,
                };

                if antipodal.x < end.x {
                    (1, 0)
                } else {
                    (0, 0)
                }
            }
            ("middle", "middle") if start.x <= end.x && start.y >= end.y => {
                let antipodal = PointS {
                    x: start.x + (center.x - start.x) * 2,
                    y: start.y + (center.y - start.y) * 2,
                };

                if antipodal.x < end.x {
                    (0, 0)
                } else {
                    (1, 0)
                }
            }
            ("middle", "middle") if start.x >= end.x && start.y <= end.y => {
                let antipodal = PointS {
                    x: start.x - (center.x - end.x) * 2,
                    y: start.y - (center.y - end.y) * 2,
                };

                if antipodal.x < end.x {
                    (1, 0)
                } else {
                    (0, 0)
                }
            }
            ("middle", "middle") if start.x >= end.x && start.y >= end.y => {
                let antipodal = PointS {
                    x: start.x - (center.x - end.x) * 2,
                    y: start.y - (center.y - end.y) * 2,
                };

                if antipodal.x < end.x {
                    (0, 0)
                } else {
                    (1, 0)
                }
            }
            _ => {
                return Err(PlayError::InvalidRecord {
                    cause: "invalid points and bounding rectangle".to_owned(),
                });
            }
        };

        let data = Data::new()
            .move_to(format!("{} {}", start.x, start.y))
            .elliptical_arc_to(format!(
                "{} {} {} {} {} {} {}",
                rx, ry, 0, large_arc, sweep, end.x, end.y
            ));
        let path = Node::node("path").set("fill", "none").set("d", data);
        let path = stroke.set_props(path);

        self.set_current_context(context.drawing_position(end));
        self.elements.push(path.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn chord(&mut self, record: META_CHORD) -> Result<(), PlayError> {
        tracing::info!("META_CHORD: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ellipse(&mut self, record: META_ELLIPSE) -> Result<(), PlayError> {
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

        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()).into());
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let point = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.left_rect + rx,
                y: record.top_rect + ry,
            });

            context = context.extend_window(&point);
            point
        };

        let ellipse = Node::node("ellipse")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("cx", point.x)
            .set("cy", point.y)
            .set("rx", rx)
            .set("ry", ry);
        let ellipse = stroke.set_props(ellipse);

        self.set_current_context(context);
        self.elements.push(ellipse.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_flood_fill(
        &mut self,
        record: META_EXTFLOODFILL,
    ) -> Result<(), PlayError> {
        tracing::info!("META_EXTFLOODFILL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_text_out(
        &mut self,
        record: META_EXTTEXTOUT,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let font = self.selected_font()?;
        let point = {
            let point = PointS {
                x: record.x,
                y: record.y
                    + (if matches!(
                        context.text_align_vertical,
                        VerticalTextAlignmentMode::VTA_BASELINE
                            | VerticalTextAlignmentMode::VTA_BOTTOM
                    ) && font.height < 0
                    {
                        -font.height
                    } else {
                        0
                    }),
            };

            let point = if context.text_align_update_cp {
                context.point_s_to_relative_point(&point)
            } else {
                context.point_s_to_absolute_point(&point)
            };

            context = context.extend_window(&point);
            point
        };
        let text_align = context.as_css_text_align();
        let shape_inside = if let (true, Some(rect)) = (
            record.fw_opts.contains(&ExtTextOutOptions::ETO_CLIPPED),
            record.rectangle,
        ) {
            let tl = {
                let point = PointS { x: rect.left, y: rect.top };
                let point = if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                };

                context = context.extend_window(&point);
                point
            };
            let tr = {
                let point = PointS { x: rect.right, y: rect.top };
                let point = if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                };

                context = context.extend_window(&point);
                point
            };
            let bl = {
                let point = PointS { x: rect.left, y: rect.bottom };
                let point = if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                };

                context = context.extend_window(&point);
                point
            };
            let br = {
                let point = PointS { x: rect.right, y: rect.bottom };
                let point = if context.text_align_update_cp {
                    context.point_s_to_relative_point(&point)
                } else {
                    context.point_s_to_absolute_point(&point)
                };

                context = context.extend_window(&point);
                point
            };

            Some(format!(
                "shape-inside: polygon({} {} {} {});",
                as_point_string(&tl),
                as_point_string(&bl),
                as_point_string(&br),
                as_point_string(&tr),
            ))
        } else {
            None
        };

        let text = Node::node("text")
            .set("x", point.x)
            .set("y", point.y)
            .set("text-anchor", text_align)
            .set("dominant-baseline", context.as_css_text_align_vertical())
            .set("fill", context.text_color_as_css_color())
            .add(Node::text(record.string));
        let (text, mut styles) = font.set_props(text, &point);

        if let Some(shape_inside) = shape_inside {
            styles.push(shape_inside);
        }

        let text = text.set("style", styles.join(""));

        if context.text_align_update_cp {
            context = context.drawing_position(point);
        }

        self.set_current_context(context);
        self.elements.push(text.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn fill_region(
        &mut self,
        record: META_FILLREGION,
    ) -> Result<(), PlayError> {
        tracing::info!("META_FILLREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn flood_fill(&mut self, record: META_FLOODFILL) -> Result<(), PlayError> {
        tracing::info!("META_FLOODFILL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn frame_region(
        &mut self,
        record: META_FRAMEREGION,
    ) -> Result<(), PlayError> {
        tracing::info!("META_FRAMEREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn invert_region(
        &mut self,
        record: META_INVERTREGION,
    ) -> Result<(), PlayError> {
        tracing::info!("META_INVERTREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn line_to(&mut self, record: META_LINETO) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());
        let point = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.x,
                y: record.y,
            });

            context = context.extend_window(&point);
            point
        };

        let data = Data::new()
            .move_to(format!(
                "{} {}",
                context.drawing_position.x, context.drawing_position.y
            ))
            .line_to(format!("{} {}", point.x, point.y));
        let path = Node::node("path").set("fill", "none").set("d", data);
        let path = stroke.set_props(path);

        self.set_current_context(context.drawing_position(point));
        self.elements.push(path.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn paint_region(
        &mut self,
        _record: META_PAINTREGION,
    ) -> Result<(), PlayError> {
        tracing::info!("META_PAINTREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn pat_blt(&mut self, record: META_PATBLT) -> Result<(), PlayError> {
        if record.width == 0 || record.height == 0 {
            tracing::info!(
                %record.width,
                %record.height,
                "META_PATBLT is skipped because width or height is zero.",
            );

            return Ok(());
        }

        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()).into());
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.current_context().poly_fill_rule();

        let rect = Node::node("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", "none")
            .set("x", record.x_left)
            .set("y", record.y_left)
            .set("height", record.height)
            .set("width", record.width);

        self.elements.push(rect.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn pie(&mut self, record: META_PIE) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let brush = self.selected_brush();
        let stroke = Stroke::from(brush.clone());
        let fill = match Fill::from(brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()).into());
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let (rx, ry) = (
            (record.right_rect - record.left_rect) / 2,
            (record.bottom_rect - record.top_rect) / 2,
        );
        let (center_x, center_y) =
            (record.left_rect + rx, record.top_rect + ry);

        let ellipse = Node::node("ellipse")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("cx", center_x)
            .set("cy", center_y)
            .set("rx", rx)
            .set("ry", ry);
        let ellipse = stroke.set_props(ellipse);

        let stroke = Stroke::from(self.selected_pen().clone());
        let p1 = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.x_radial1,
                y: record.y_radial1,
            });

            context = context.extend_window(&point);
            point
        };
        let center = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: center_x,
                y: center_y,
            });
            context = context.extend_window(&point);
            point
        };
        let p2 = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.x_radial2,
                y: record.y_radial2,
            });

            context = context.extend_window(&point);
            point
        };

        let data = Data::new()
            .move_to(format!("{} {}", p1.x, p1.y))
            .line_to(format!("{} {}", center.x, center.y))
            .line_to(format!("{} {}", p2.x, p2.y));
        let path = Node::node("path").set("fill", "none").set("d", data);
        let path = stroke.set_props(path);

        self.set_current_context(context.drawing_position(p2));
        self.elements.push(ellipse.into());
        self.elements.push(path.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polyline(&mut self, record: META_POLYLINE) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());

        let Some(point) = record.a_points.first() else {
            return Err(PlayError::InvalidRecord {
                cause: "aPoints[0] is not defined".to_owned(),
            });
        };

        let mut coordinate = {
            let point = context.point_s_to_absolute_point(point);
            context = context.extend_window(&point);
            point
        };

        let mut data =
            Data::new().move_to(format!("{} {}", coordinate.x, coordinate.y));

        for i in 1..record.number_of_points {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            coordinate = {
                let point = context.point_s_to_absolute_point(point);
                context = context.extend_window(&point);
                point
            };

            data = data.line_to(format!("{} {}", coordinate.x, coordinate.y));
        }

        let path = Node::node("path").set("fill", "none").set("d", data);
        let path = stroke.set_props(path);

        self.set_current_context(context.drawing_position(coordinate));
        self.elements.push(path.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polygon(&mut self, record: META_POLYGON) -> Result<(), PlayError> {
        if record.number_of_points == 0 {
            tracing::info!(%record.number_of_points, "polygon has no points");
            return Ok(());
        }

        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()).into());
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();

        let mut points = vec![];

        for i in 0..record.number_of_points {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            let point = {
                let point = context.point_s_to_absolute_point(point);
                context = context.extend_window(&point);
                point
            };

            points.push(as_point_string(&point));
        }

        let polygon = Node::node("polygon")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("points", points.join(" "));
        let polygon = stroke.set_props(polygon);

        self.set_current_context(context);
        self.elements.push(polygon.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_polygon(
        &mut self,
        record: META_POLYPOLYGON,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());
        tracing::debug!(?stroke, "Stroke from selected Pen");
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()).into());
                url_string(format!("#{id}").as_str())
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
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPointsPerPolygon[{i}] is not defined"),
                });
            };

            let mut points = vec![];

            for _ in 0..*points_of_polygon {
                let Some(point) = a_point.pop_front() else {
                    return Err(PlayError::InvalidRecord {
                        cause: format!(
                            "aPoints[{current_point_index}] is not defined"
                        ),
                    });
                };

                let point = {
                    let point = context.point_s_to_absolute_point(&point);
                    context = context.extend_window(&point);
                    point
                };

                points.push(as_point_string(&point));
                current_point_index += 1;
            }

            let polygon = Node::node("polygon")
                .set("fill", fill.as_str())
                .set("fill-rule", fill_rule.as_str())
                .set("points", points.join(" "));
            let polygon = stroke.set_props(polygon);

            self.elements.push(polygon.into());
        }

        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn reactangle(&mut self, record: META_RECTANGLE) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()).into());
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let tl = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.left_rect,
                y: record.top_rect,
            });

            context = context.extend_window(&point);
            point
        };
        let br = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.right_rect,
                y: record.bottom_rect,
            });

            context = context.extend_window(&point);
            point
        };

        let rect = Node::node("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("x", tl.x)
            .set("y", tl.y)
            .set("height", br.y - tl.y)
            .set("width", br.x - tl.x);
        let rect = stroke.set_props(rect);

        self.set_current_context(context);
        self.elements.push(rect.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn round_rect(&mut self, record: META_ROUNDRECT) -> Result<(), PlayError> {
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

        let mut context = self.current_context().clone();
        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()).into());
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = context.poly_fill_rule();
        let point = {
            let point = context.point_s_to_absolute_point(&PointS {
                x: record.left_rect,
                y: record.top_rect,
            });

            context = context.extend_window(&point);
            point
        };

        let rect = Node::node("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("x", point.x)
            .set("y", point.y)
            .set("height", height)
            .set("width", width)
            .set("rx", record.width)
            .set("ry", record.height);
        let rect = stroke.set_props(rect);

        self.set_current_context(context);
        self.elements.push(rect.into());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_pixel(&mut self, _record: META_SETPIXEL) -> Result<(), PlayError> {
        tracing::info!("META_SETPIXEL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn text_out(&mut self, record: META_TEXTOUT) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let font = self.selected_font()?;
        let point = {
            let point = PointS {
                x: record.x_start,
                y: record.y_start
                    + (if matches!(
                        context.text_align_vertical,
                        VerticalTextAlignmentMode::VTA_BASELINE
                            | VerticalTextAlignmentMode::VTA_BOTTOM
                    ) && font.height < 0
                    {
                        -font.height
                    } else {
                        0
                    }),
            };

            let point = if context.text_align_update_cp {
                context.point_s_to_relative_point(&point)
            } else {
                context.point_s_to_absolute_point(&point)
            };

            context = context.extend_window(&point);
            point
        };

        let text = Node::node("text")
            .set("x", point.x)
            .set("y", point.y)
            .set("fill", context.text_color_as_css_color())
            .add(Node::text(record.string));
        let (text, styles) = font.set_props(text, &point);
        let text = text.set("style", styles.join(""));

        self.set_current_context(context);
        self.elements.push(text.into());

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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_brush_indirect(
        &mut self,
        record: META_CREATEBRUSHINDIRECT,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.push(GraphicsObject::Brush(record.create_brush()));
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_font_indirect(
        &mut self,
        record: META_CREATEFONTINDIRECT,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.push(GraphicsObject::Font(record.font));
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_palette(
        &mut self,
        record: META_CREATEPALETTE,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.push(GraphicsObject::Palette(record.palette));
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_pattern_brush(
        &mut self,
        record: META_CREATEPATTERNBRUSH,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.push(GraphicsObject::Brush(record.create_brush()));
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_pen_indirect(
        &mut self,
        record: META_CREATEPENINDIRECT,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.push(GraphicsObject::Pen(record.pen));
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_region(
        &mut self,
        record: META_CREATEREGION,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.push(GraphicsObject::Region(record.region));
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn delete_object(
        &mut self,
        record: META_DELETEOBJECT,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.delete(record.object_index as usize);
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_device_independent_bitmap_pattern_brush(
        &mut self,
        record: META_DIBCREATEPATTERNBRUSH,
    ) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();

        context.object_table.push(GraphicsObject::Brush(record.create_brush()));
        self.set_current_context(context);

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn select_clip_region(
        &mut self,
        _record: META_SELECTCLIPREGION,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SELECTCLIPREGION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn select_object(
        &mut self,
        record: META_SELECTOBJECT,
    ) -> Result<(), PlayError> {
        let object = self
            .current_context()
            .object_table
            .get(record.object_index as usize)
            .clone();
        let selected = self.object_selected.clone();

        self.object_selected = match object {
            GraphicsObject::Brush(v) => selected.brush(v),
            GraphicsObject::Font(v) => selected.font(v),
            GraphicsObject::Palette(v) => selected.palette(v),
            GraphicsObject::Pen(v) => selected.pen(v),
            GraphicsObject::Region(v) => selected.region(v),
            GraphicsObject::Null => {
                return Err(PlayError::UnexpectedGraphicsObject {
                    cause: "Graphics Object is null".to_owned(),
                })
            }
        };

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn select_palette(
        &mut self,
        record: META_SELECTPALETTE,
    ) -> Result<(), PlayError> {
        let object = self
            .current_context()
            .object_table
            .get(record.palette as usize)
            .clone();

        let GraphicsObject::Palette(palette) = object else {
            return Err(PlayError::UnexpectedGraphicsObject {
                cause: "Graphics Object is not palette object".to_owned(),
            });
        };

        self.object_selected = self.object_selected.clone().palette(palette);

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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn animate_palette(
        &mut self,
        _record: META_ANIMATEPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("META_ANIMATEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn exclude_clip_rect(
        &mut self,
        _record: META_EXCLUDECLIPRECT,
    ) -> Result<(), PlayError> {
        tracing::info!("META_EXCLUDECLIPRECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn intersect_clip_rect(
        &mut self,
        record: META_INTERSECTCLIPRECT,
    ) -> Result<(), PlayError> {
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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn move_to(&mut self, record: META_MOVETO) -> Result<(), PlayError> {
        let mut context = self.current_context().clone();
        let point = context
            .point_s_to_absolute_point(&PointS { x: record.x, y: record.y });
        context = context.extend_window(&point);

        self.set_current_context(context.drawing_position(point));

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn offset_clip_region(
        &mut self,
        _record: META_OFFSETCLIPRGN,
    ) -> Result<(), PlayError> {
        tracing::info!("META_OFFSETCLIPRGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn offset_viewport_origin(
        &mut self,
        _record: META_OFFSETVIEWPORTORG,
    ) -> Result<(), PlayError> {
        tracing::info!("META_OFFSETVIEWPORTORG: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn offset_window_origin(
        &mut self,
        _record: META_OFFSETWINDOWORG,
    ) -> Result<(), PlayError> {
        tracing::info!("META_OFFSETWINDOWORG: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn realize_palette(
        &mut self,
        _record: META_REALIZEPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("META_REALIZEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn resize_palette(
        &mut self,
        _record: META_RESIZEPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("META_RESIZEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn restore_device_context(
        &mut self,
        record: META_RESTOREDC,
    ) -> Result<(), PlayError> {
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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn save_device_context(
        &mut self,
        _record: META_SAVEDC,
    ) -> Result<(), PlayError> {
        self.context_stack.push(self.current_context().clone());

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn scale_viewport_ext(
        &mut self,
        _record: META_SCALEVIEWPORTEXT,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SCALEVIEWPORTEXT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn scale_window_ext(
        &mut self,
        record: META_SCALEWINDOWEXT,
    ) -> Result<(), PlayError> {
        let context = self.current_context();
        let scale_x = (context.window.scale_x * f32::from(record.x_num))
            / f32::from(record.x_denom);
        let scale_y = (context.window.scale_y * f32::from(record.y_num))
            / f32::from(record.y_denom);

        self.set_current_context(
            context.clone().window_scale(scale_x, scale_y),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_bk_color(
        &mut self,
        record: META_SETBKCOLOR,
    ) -> Result<(), PlayError> {
        self.set_current_context(
            self.current_context().clone().text_bk_color(record.color_ref),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_bk_mode(&mut self, record: META_SETBKMODE) -> Result<(), PlayError> {
        self.set_current_context(
            self.current_context().clone().bk_mode(record.bk_mode),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_layout(&mut self, _record: META_SETLAYOUT) -> Result<(), PlayError> {
        tracing::info!("META_SETLAYOUT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_map_mode(
        &mut self,
        record: META_SETMAPMODE,
    ) -> Result<(), PlayError> {
        self.set_current_context(
            self.current_context().clone().map_mode(record.map_mode),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_mapper_flags(
        &mut self,
        _record: META_SETMAPPERFLAGS,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETMAPPERFLAGS: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_pal_entries(
        &mut self,
        _record: META_SETPALENTRIES,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETPALENTRIES: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_polyfill_mode(
        &mut self,
        record: META_SETPOLYFILLMODE,
    ) -> Result<(), PlayError> {
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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_relabs(&mut self, _record: META_SETRELABS) -> Result<(), PlayError> {
        tracing::info!("META_SETRELABS: reserved record and not supported");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_raster_operation(
        &mut self,
        record: META_SETROP2,
    ) -> Result<(), PlayError> {
        self.set_current_context(
            self.current_context().clone().draw_mode(record.draw_mode),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_stretch_blt_mode(
        &mut self,
        _record: META_SETSTRETCHBLTMODE,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETSTRETCHBLTMODE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_text_align(
        &mut self,
        record: META_SETTEXTALIGN,
    ) -> Result<(), PlayError> {
        let update_cp = record.text_alignment_mode
            & (TextAlignmentMode::TA_UPDATECP as u16)
            == TextAlignmentMode::TA_UPDATECP as u16;
        let align_horizontal =
            [TextAlignmentMode::TA_CENTER, TextAlignmentMode::TA_RIGHT]
                .into_iter()
                .find(|a| record.text_alignment_mode & (*a as u16) == *a as u16)
                .unwrap_or(TextAlignmentMode::TA_LEFT);
        let align_vertical = [
            VerticalTextAlignmentMode::VTA_BOTTOM,
            VerticalTextAlignmentMode::VTA_TOP,
        ]
        .into_iter()
        .find(|a| record.text_alignment_mode & (*a as u16) == *a as u16)
        .unwrap_or(VerticalTextAlignmentMode::VTA_BASELINE);

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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_text_char_extra(
        &mut self,
        _record: META_SETTEXTCHAREXTRA,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETTEXTCHAREXTRA: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_text_color(
        &mut self,
        record: META_SETTEXTCOLOR,
    ) -> Result<(), PlayError> {
        self.set_current_context(
            self.current_context().clone().text_color(record.color_ref),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_text_justification(
        &mut self,
        _record: META_SETTEXTJUSTIFICATION,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETTEXTJUSTIFICATION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_viewport_ext(
        &mut self,
        _record: META_SETVIEWPORTEXT,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETVIEWPORTEXT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_viewport_origin(
        &mut self,
        _record: META_SETVIEWPORTORG,
    ) -> Result<(), PlayError> {
        tracing::info!("META_SETVIEWPORTORG: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_window_ext(
        &mut self,
        record: META_SETWINDOWEXT,
    ) -> Result<(), PlayError> {
        self.set_current_context(
            self.current_context().clone().window_ext(record.x, record.y),
        );

        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_window_origin(
        &mut self,
        record: META_SETWINDOWORG,
    ) -> Result<(), PlayError> {
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
        err(level = tracing::Level::ERROR, Display),
    )]
    fn escape(&mut self, _record: META_ESCAPE) -> Result<(), PlayError> {
        Ok(())
    }
}
