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

#[derive(Default)]
pub struct SVGPlayer {
    context_stack: Vec<DeviceContext>,
    context_current: DeviceContext,
    definitions: Vec<Node>,
    elements: Vec<Node>,
    object_selected: SelectedGraphicsObject,
}

impl SVGPlayer {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn issue_definition_id(&self) -> String {
        format!("defs{}", self.definitions.len())
    }

    #[inline]
    fn push_element(&mut self, record_number: usize, mut element: Node) {
        if record_number > 0 {
            element = element.set("id", format!("elem{record_number}"));
        }

        self.elements.push(element);
    }

    fn selected_brush(&self) -> &Brush {
        &self.object_selected.brush
    }

    fn selected_pen(&self) -> &Pen {
        &self.object_selected.pen
    }
}

impl crate::converter::Player for SVGPlayer {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        let Self { context_current, definitions, elements, .. } = self;

        let (x, y, width, height) = context_current.window.as_view_box();
        let mut document = Node::new("svg")
            .set("xmlns", "http://www.w3.org/2000/svg")
            .set("viewBox", format!("{x} {y} {width} {height}"));

        if !definitions.is_empty() {
            let mut defs = Node::new("defs");
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
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn selected_font(&self) -> Result<&Font, PlayError> {
        Ok(&self.object_selected.font)
    }

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn bit_blt(
        mut self,
        record_number: usize,
        record: META_BITBLT,
    ) -> Result<Self, PlayError> {
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
            return Ok(self);
        };

        self.push_element(record_number, elem);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn device_independent_bitmap_bit_blt(
        mut self,
        record_number: usize,
        record: META_DIBBITBLT,
    ) -> Result<Self, PlayError> {
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
            return Ok(self);
        };

        self.push_element(record_number, elem);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn device_independent_bitmap_stretch_blt(
        mut self,
        record_number: usize,
        record: META_DIBSTRETCHBLT,
    ) -> Result<Self, PlayError> {
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
            return Ok(self);
        };

        self.push_element(record_number, elem);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_device_independent_bitmap_to_dev(
        self,
        record_number: usize,
        record: META_SETDIBTODEV,
    ) -> Result<Self, PlayError> {
        info!("META_SETDIBTODEV: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stretch_blt(
        mut self,
        record_number: usize,
        record: META_STRETCHBLT,
    ) -> Result<Self, PlayError> {
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
            return Ok(self);
        };

        self.push_element(record_number, elem);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stretch_device_independent_bitmap(
        mut self,
        record_number: usize,
        record: META_STRETCHDIB,
    ) -> Result<Self, PlayError> {
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
            return Ok(self);
        };

        self.push_element(record_number, elem);

        Ok(self)
    }

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn eof(self, record_number: usize, _: META_EOF) -> Result<Self, PlayError> {
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn header(
        mut self,
        record_number: usize,
        header: MetafileHeader,
    ) -> Result<Self, PlayError> {
        let (_placeable, header) = match header {
            MetafileHeader::StartsWithHeader(header) => (None, header),
            MetafileHeader::StartsWithPlaceable(placeable, header) => {
                (Some(placeable), header)
            }
        };

        self.context_current =
            self.context_current.create_object_table(header.number_of_objects);

        Ok(self)
    }

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn arc(
        mut self,
        record_number: usize,
        record: META_ARC,
    ) -> Result<Self, PlayError> {
        let stroke = Stroke::from(self.selected_pen().clone());
        let start = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.x_start_arc,
                    y: record.y_start_arc,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };
        let end = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.x_end_arc,
                    y: record.y_end_arc,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };
        let (rx, ry) = (
            (record.right_rect - record.left_rect) / 2,
            (record.bottom_rect - record.top_rect) / 2,
        );
        let center = self.context_current.point_s_to_absolute_point(&PointS {
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
        let path =
            Node::new("path").set("fill", "none").set("d", data.to_string());
        let path = stroke.set_props(path);

        self.context_current = self.context_current.drawing_position(end);
        self.push_element(record_number, path);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn chord(
        self,
        record_number: usize,
        record: META_CHORD,
    ) -> Result<Self, PlayError> {
        info!("META_CHORD: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ellipse(
        mut self,
        record_number: usize,
        record: META_ELLIPSE,
    ) -> Result<Self, PlayError> {
        let (rx, ry) = (
            (record.right_rect - record.left_rect) / 2,
            (record.bottom_rect - record.top_rect) / 2,
        );

        if rx == 0 || ry == 0 {
            info!(
                %rx, %ry,
                "META_ELLIPSE is skipped because rx or ry is zero.",
            );

            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.context_current.poly_fill_rule();
        let point = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.left_rect + rx,
                    y: record.top_rect + ry,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };

        let ellipse = Node::new("ellipse")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("cx", point.x.to_string())
            .set("cy", point.y.to_string())
            .set("rx", rx.to_string())
            .set("ry", ry.to_string());
        let ellipse = stroke.set_props(ellipse);

        self.push_element(record_number, ellipse);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_flood_fill(
        self,
        record_number: usize,
        record: META_EXTFLOODFILL,
    ) -> Result<Self, PlayError> {
        info!("META_EXTFLOODFILL: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_text_out(
        mut self,
        record_number: usize,
        record: META_EXTTEXTOUT,
    ) -> Result<Self, PlayError> {
        use unicode_segmentation::UnicodeSegmentation;

        let font_height = self.selected_font()?.height;
        let point = {
            let point = PointS {
                x: if self.context_current.text_align_update_cp {
                    self.context_current.drawing_position.x
                } else {
                    record.x
                },
                y: if self.context_current.text_align_update_cp {
                    self.context_current.drawing_position.y
                } else {
                    record.y
                } + (if matches!(
                    self.context_current.text_align_vertical,
                    VerticalTextAlignmentMode::VTA_BASELINE
                        | VerticalTextAlignmentMode::VTA_BOTTOM
                ) && font_height < 0
                {
                    -font_height
                } else {
                    0
                }),
            };

            let point = if self.context_current.text_align_update_cp {
                point
            } else {
                self.context_current.point_s_to_absolute_point(&point)
            };

            self.context_current = self.context_current.extend_window(&point);
            point
        };
        let text_align = self.context_current.as_css_text_align();
        let shape_inside = if let (true, Some(rect)) = (
            record.fw_opts.contains(&ExtTextOutOptions::ETO_CLIPPED),
            record.rectangle,
        ) {
            let tl = {
                let point = PointS { x: rect.left, y: rect.top };
                let point = if self.context_current.text_align_update_cp {
                    self.context_current.point_s_to_relative_point(&point)
                } else {
                    self.context_current.point_s_to_absolute_point(&point)
                };

                self.context_current =
                    self.context_current.extend_window(&point);
                point
            };
            let tr = {
                let point = PointS { x: rect.right, y: rect.top };
                let point = if self.context_current.text_align_update_cp {
                    self.context_current.point_s_to_relative_point(&point)
                } else {
                    self.context_current.point_s_to_absolute_point(&point)
                };

                self.context_current =
                    self.context_current.extend_window(&point);
                point
            };
            let bl = {
                let point = PointS { x: rect.left, y: rect.bottom };
                let point = if self.context_current.text_align_update_cp {
                    self.context_current.point_s_to_relative_point(&point)
                } else {
                    self.context_current.point_s_to_absolute_point(&point)
                };

                self.context_current =
                    self.context_current.extend_window(&point);
                point
            };
            let br = {
                let point = PointS { x: rect.right, y: rect.bottom };
                let point = if self.context_current.text_align_update_cp {
                    self.context_current.point_s_to_relative_point(&point)
                } else {
                    self.context_current.point_s_to_absolute_point(&point)
                };

                self.context_current =
                    self.context_current.extend_window(&point);
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

        let mut text = Node::new("text")
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("text-anchor", text_align)
            .set(
                "dominant-baseline",
                self.context_current.as_css_text_align_vertical(),
            )
            .set("fill", self.context_current.text_color_as_css_color())
            .add(Node::new_text(record.string.as_str()));

        // https://opengrok.libreoffice.org/xref/core/emfio/source/reader/wmfreader.cxx?r=07a3dd72f3eb79c03297aa9af9d77326b07458b6#693-826
        if !record.dx.is_empty()
            && record.string.graphemes(true).count() == record.dx.len()
        {
            let mut dx = Vec::with_capacity(record.dx.len() + 1);

            dx.push(0);
            dx.extend(record.dx);

            text = text.set(
                "dx",
                dx.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }

        let (text, mut styles) = self.selected_font()?.set_props(text, &point);

        if let Some(shape_inside) = shape_inside {
            styles.push(shape_inside);
        }

        let mut text = text.set("style", styles.join(""));

        if self.context_current.text_align_update_cp {
            self.context_current =
                self.context_current.drawing_position(point.clone());
        }

        // HACK: background color
        // https://stackoverflow.com/a/31013492
        if self.context_current.bk_mode == MixMode::OPAQUE {
            let id = self.issue_definition_id();
            let brush = if matches!(self.selected_brush(), Brush::Null) {
                Brush::Solid {
                    color_ref: self.context_current.text_bk_color.clone(),
                }
            } else {
                self.selected_brush().clone()
            };

            self.definitions.push(brush.as_filter().set("id", id.as_str()));

            text = text.set("filter", url_string(format!("#{id}").as_str()));
        }

        self.push_element(record_number, text);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn fill_region(
        self,
        record_number: usize,
        record: META_FILLREGION,
    ) -> Result<Self, PlayError> {
        info!("META_FILLREGION: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn flood_fill(
        self,
        record_number: usize,
        record: META_FLOODFILL,
    ) -> Result<Self, PlayError> {
        info!("META_FLOODFILL: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn frame_region(
        self,
        record_number: usize,
        record: META_FRAMEREGION,
    ) -> Result<Self, PlayError> {
        info!("META_FRAMEREGION: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn invert_region(
        self,
        record_number: usize,
        record: META_INVERTREGION,
    ) -> Result<Self, PlayError> {
        info!("META_INVERTREGION: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn line_to(
        mut self,
        record_number: usize,
        record: META_LINETO,
    ) -> Result<Self, PlayError> {
        let stroke = Stroke::from(self.selected_pen().clone());
        let point = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.x,
                    y: record.y,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };

        let data = Data::new()
            .move_to(format!(
                "{} {}",
                self.context_current.drawing_position.x,
                self.context_current.drawing_position.y
            ))
            .line_to(format!("{} {}", point.x, point.y));
        let path =
            Node::new("path").set("fill", "none").set("d", data.to_string());
        let path = stroke.set_props(path);

        self.context_current = self.context_current.drawing_position(point);
        self.push_element(record_number, path);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn paint_region(
        self,
        record_number: usize,
        record: META_PAINTREGION,
    ) -> Result<Self, PlayError> {
        info!("META_PAINTREGION: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn pat_blt(
        mut self,
        record_number: usize,
        record: META_PATBLT,
    ) -> Result<Self, PlayError> {
        if record.width == 0 || record.height == 0 {
            info!(
                %record.width,
                %record.height,
                "META_PATBLT is skipped because width or height is zero.",
            );

            return Ok(self);
        }

        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.context_current.poly_fill_rule();

        let rect = Node::new("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("stroke", "none")
            .set("x", record.x_left.to_string())
            .set("y", record.y_left.to_string())
            .set("height", record.height.to_string())
            .set("width", record.width.to_string());

        self.push_element(record_number, rect);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn pie(
        mut self,
        record_number: usize,
        record: META_PIE,
    ) -> Result<Self, PlayError> {
        let brush = self.selected_brush();
        let stroke = Stroke::from(brush.clone());
        let fill = match Fill::from(brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.context_current.poly_fill_rule();
        let (rx, ry) = (
            (record.right_rect - record.left_rect) / 2,
            (record.bottom_rect - record.top_rect) / 2,
        );
        let (center_x, center_y) =
            (record.left_rect + rx, record.top_rect + ry);

        let ellipse = Node::new("ellipse")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("cx", center_x.to_string())
            .set("cy", center_y.to_string())
            .set("rx", rx.to_string())
            .set("ry", ry.to_string());
        let ellipse = stroke.set_props(ellipse);

        let stroke = Stroke::from(self.selected_pen().clone());
        let p1 = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.x_radial1,
                    y: record.y_radial1,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };
        let center = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: center_x,
                    y: center_y,
                });
            self.context_current = self.context_current.extend_window(&point);
            point
        };
        let p2 = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.x_radial2,
                    y: record.y_radial2,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };

        let data = Data::new()
            .move_to(format!("{} {}", p1.x, p1.y))
            .line_to(format!("{} {}", center.x, center.y))
            .line_to(format!("{} {}", p2.x, p2.y));
        let path =
            Node::new("path").set("fill", "none").set("d", data.to_string());
        let path = stroke.set_props(path);

        self.context_current = self.context_current.drawing_position(p2);
        self.push_element(record_number, ellipse);
        self.push_element(record_number, path);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline(
        mut self,
        record_number: usize,
        record: META_POLYLINE,
    ) -> Result<Self, PlayError> {
        let stroke = Stroke::from(self.selected_pen().clone());
        let Some(point) = record.a_points.first() else {
            return Err(PlayError::InvalidRecord {
                cause: "aPoints[0] is not defined".to_owned(),
            });
        };

        let mut coordinate = {
            let point = self.context_current.point_s_to_absolute_point(point);
            self.context_current = self.context_current.extend_window(&point);
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
                let point =
                    self.context_current.point_s_to_absolute_point(point);
                self.context_current =
                    self.context_current.extend_window(&point);
                point
            };

            data = data.line_to(format!("{} {}", coordinate.x, coordinate.y));
        }

        let path =
            Node::new("path").set("fill", "none").set("d", data.to_string());
        let path = stroke.set_props(path);

        self.context_current =
            self.context_current.drawing_position(coordinate);
        self.push_element(record_number, path);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polygon(
        mut self,
        record_number: usize,
        record: META_POLYGON,
    ) -> Result<Self, PlayError> {
        if record.number_of_points == 0 {
            info!(%record.number_of_points, "polygon has no points");
            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.context_current.poly_fill_rule();

        let mut points = Vec::with_capacity(record.number_of_points as usize);

        for i in 0..record.number_of_points {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            let point = {
                let point =
                    self.context_current.point_s_to_absolute_point(point);
                self.context_current =
                    self.context_current.extend_window(&point);
                point
            };

            points.push(as_point_string(&point));
        }

        let polygon = Node::new("polygon")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("points", points.join(" "));
        let polygon = stroke.set_props(polygon);

        self.push_element(record_number, polygon);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polygon(
        mut self,
        record_number: usize,
        record: META_POLYPOLYGON,
    ) -> Result<Self, PlayError> {
        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.context_current.poly_fill_rule();

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

            let mut points = Vec::with_capacity(*points_of_polygon as usize);

            for _ in 0..*points_of_polygon {
                let Some(point) = a_point.pop_front() else {
                    return Err(PlayError::InvalidRecord {
                        cause: format!(
                            "aPoints[{current_point_index}] is not defined"
                        ),
                    });
                };

                let point = {
                    let point =
                        self.context_current.point_s_to_absolute_point(&point);
                    self.context_current =
                        self.context_current.extend_window(&point);
                    point
                };

                points.push(as_point_string(&point));
                current_point_index += 1;
            }

            let polygon = Node::new("polygon")
                .set("fill", fill.as_str())
                .set("fill-rule", fill_rule.as_str())
                .set("points", points.join(" "));
            let polygon = stroke.set_props(polygon);

            self.push_element(record_number, polygon);
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn rectangle(
        mut self,
        record_number: usize,
        record: META_RECTANGLE,
    ) -> Result<Self, PlayError> {
        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.context_current.poly_fill_rule();
        let tl = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.left_rect,
                    y: record.top_rect,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };
        let br = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.right_rect,
                    y: record.bottom_rect,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };

        let rect = Node::new("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("x", tl.x.to_string())
            .set("y", tl.y.to_string())
            .set("height", (br.y - tl.y).to_string())
            .set("width", (br.x - tl.x).to_string());
        let rect = stroke.set_props(rect);

        self.push_element(record_number, rect);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn round_rect(
        mut self,
        record_number: usize,
        record: META_ROUNDRECT,
    ) -> Result<Self, PlayError> {
        let (width, height) = (
            record.right_rect - record.left_rect,
            record.bottom_rect - record.top_rect,
        );

        if width == 0 || height == 0 {
            info!(
                %width, %height,
                "META_ROUNDRECT is skipped because width or height is zero.",
            );

            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_pen().clone());
        let fill = match Fill::from(self.selected_brush().clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = self.context_current.poly_fill_rule();
        let point = {
            let point =
                self.context_current.point_s_to_absolute_point(&PointS {
                    x: record.left_rect,
                    y: record.top_rect,
                });

            self.context_current = self.context_current.extend_window(&point);
            point
        };

        let rect = Node::new("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("height", height.to_string())
            .set("width", width.to_string())
            .set("rx", record.width.to_string())
            .set("ry", record.height.to_string());
        let rect = stroke.set_props(rect);

        self.push_element(record_number, rect);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_pixel(
        self,
        record_number: usize,
        record: META_SETPIXEL,
    ) -> Result<Self, PlayError> {
        info!("META_SETPIXEL: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn text_out(
        mut self,
        record_number: usize,
        record: META_TEXTOUT,
    ) -> Result<Self, PlayError> {
        let point = {
            let font_height = self.selected_font()?.height;
            let point = PointS {
                x: record.x_start,
                y: record.y_start
                    + (if matches!(
                        self.context_current.text_align_vertical,
                        VerticalTextAlignmentMode::VTA_BASELINE
                            | VerticalTextAlignmentMode::VTA_BOTTOM
                    ) && font_height < 0
                    {
                        -font_height
                    } else {
                        0
                    }),
            };

            let point = if self.context_current.text_align_update_cp {
                self.context_current.point_s_to_relative_point(&point)
            } else {
                self.context_current.point_s_to_absolute_point(&point)
            };

            self.context_current = self.context_current.extend_window(&point);
            point
        };

        let text = Node::new("text")
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("fill", self.context_current.text_color_as_css_color())
            .add(Node::new_text(record.string));
        let (text, styles) = self.selected_font()?.set_props(text, &point);
        let text = text.set("style", styles.join(""));

        self.push_element(record_number, text);

        Ok(self)
    }

    // .
    // .
    // Functions to handle Object Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_brush_indirect(
        mut self,
        record_number: usize,
        record: META_CREATEBRUSHINDIRECT,
    ) -> Result<Self, PlayError> {
        self.context_current
            .object_table
            .push(GraphicsObject::Brush(record.create_brush()));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_font_indirect(
        mut self,
        record_number: usize,
        record: META_CREATEFONTINDIRECT,
    ) -> Result<Self, PlayError> {
        self.context_current
            .object_table
            .push(GraphicsObject::Font(record.font));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_palette(
        mut self,
        record_number: usize,
        record: META_CREATEPALETTE,
    ) -> Result<Self, PlayError> {
        self.context_current
            .object_table
            .push(GraphicsObject::Palette(record.palette));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_pattern_brush(
        mut self,
        record_number: usize,
        record: META_CREATEPATTERNBRUSH,
    ) -> Result<Self, PlayError> {
        self.context_current
            .object_table
            .push(GraphicsObject::Brush(record.create_brush()));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_pen_indirect(
        mut self,
        record_number: usize,
        record: META_CREATEPENINDIRECT,
    ) -> Result<Self, PlayError> {
        self.context_current.object_table.push(GraphicsObject::Pen(record.pen));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_region(
        mut self,
        record_number: usize,
        record: META_CREATEREGION,
    ) -> Result<Self, PlayError> {
        self.context_current
            .object_table
            .push(GraphicsObject::Region(record.region));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn delete_object(
        mut self,
        record_number: usize,
        record: META_DELETEOBJECT,
    ) -> Result<Self, PlayError> {
        self.context_current.object_table.delete(record.object_index as usize);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_device_independent_bitmap_pattern_brush(
        mut self,
        record_number: usize,
        record: META_DIBCREATEPATTERNBRUSH,
    ) -> Result<Self, PlayError> {
        self.context_current
            .object_table
            .push(GraphicsObject::Brush(record.create_brush()));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_clip_region(
        self,
        record_number: usize,
        record: META_SELECTCLIPREGION,
    ) -> Result<Self, PlayError> {
        info!("META_SELECTCLIPREGION: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_object(
        mut self,
        record_number: usize,
        record: META_SELECTOBJECT,
    ) -> Result<Self, PlayError> {
        let object = self
            .context_current
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

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_palette(
        mut self,
        record_number: usize,
        record: META_SELECTPALETTE,
    ) -> Result<Self, PlayError> {
        let object =
            self.context_current.object_table.get(record.palette as usize);

        let GraphicsObject::Palette(palette) = object else {
            return Err(PlayError::UnexpectedGraphicsObject {
                cause: "Graphics Object is not palette object".to_owned(),
            });
        };

        self.object_selected = self.object_selected.palette(palette.clone());

        Ok(self)
    }

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn animate_palette(
        self,
        record_number: usize,
        record: META_ANIMATEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_ANIMATEPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn exclude_clip_rect(
        self,
        record_number: usize,
        record: META_EXCLUDECLIPRECT,
    ) -> Result<Self, PlayError> {
        info!("META_EXCLUDECLIPRECT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn intersect_clip_rect(
        mut self,
        record_number: usize,
        record: META_INTERSECTCLIPRECT,
    ) -> Result<Self, PlayError> {
        let META_INTERSECTCLIPRECT { bottom, right, top, left, .. } = record;

        self.context_current = self.context_current.clipping_region(Rect {
            left,
            top,
            right,
            bottom,
        });

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn move_to(
        mut self,
        record_number: usize,
        record: META_MOVETO,
    ) -> Result<Self, PlayError> {
        let point = self
            .context_current
            .point_s_to_absolute_point(&PointS { x: record.x, y: record.y });
        self.context_current =
            self.context_current.extend_window(&point).drawing_position(point);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn offset_clip_region(
        self,
        record_number: usize,
        record: META_OFFSETCLIPRGN,
    ) -> Result<Self, PlayError> {
        info!("META_OFFSETCLIPRGN: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn offset_viewport_origin(
        self,
        record_number: usize,
        record: META_OFFSETVIEWPORTORG,
    ) -> Result<Self, PlayError> {
        info!("META_OFFSETVIEWPORTORG: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn offset_window_origin(
        self,
        record_number: usize,
        record: META_OFFSETWINDOWORG,
    ) -> Result<Self, PlayError> {
        info!("META_OFFSETWINDOWORG: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn realize_palette(
        self,
        record_number: usize,
        record: META_REALIZEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_REALIZEPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn resize_palette(
        self,
        record_number: usize,
        record: META_RESIZEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("META_RESIZEPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn restore_device_context(
        mut self,
        record_number: usize,
        record: META_RESTOREDC,
    ) -> Result<Self, PlayError> {
        let context = if record.n_saved_dc < 0 {
            self.context_current.clone().into()
        } else if (record.n_saved_dc as usize) < self.context_stack.len() {
            self.context_stack.remove(record.n_saved_dc as usize).into()
        } else {
            None
        };

        if let Some(ctx) = context {
            self.context_current = ctx;
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn save_device_context(
        mut self,
        record_number: usize,
        record: META_SAVEDC,
    ) -> Result<Self, PlayError> {
        self.context_stack.push(self.context_current.clone());

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn scale_viewport_ext(
        self,
        record_number: usize,
        record: META_SCALEVIEWPORTEXT,
    ) -> Result<Self, PlayError> {
        info!("META_SCALEVIEWPORTEXT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn scale_window_ext(
        mut self,
        record_number: usize,
        record: META_SCALEWINDOWEXT,
    ) -> Result<Self, PlayError> {
        let scale_x = (self.context_current.window.scale_x
            * f32::from(record.x_num))
            / f32::from(record.x_denom);
        let scale_y = (self.context_current.window.scale_y
            * f32::from(record.y_num))
            / f32::from(record.y_denom);

        self.context_current =
            self.context_current.window_scale(scale_x, scale_y);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_color(
        mut self,
        record_number: usize,
        record: META_SETBKCOLOR,
    ) -> Result<Self, PlayError> {
        self.context_current =
            self.context_current.text_bk_color(record.color_ref);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_mode(
        mut self,
        record_number: usize,
        record: META_SETBKMODE,
    ) -> Result<Self, PlayError> {
        self.context_current = self.context_current.bk_mode(record.bk_mode);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_layout(
        self,
        record_number: usize,
        record: META_SETLAYOUT,
    ) -> Result<Self, PlayError> {
        info!("META_SETLAYOUT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_map_mode(
        mut self,
        record_number: usize,
        record: META_SETMAPMODE,
    ) -> Result<Self, PlayError> {
        self.context_current = self.context_current.map_mode(record.map_mode);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_mapper_flags(
        self,
        record_number: usize,
        record: META_SETMAPPERFLAGS,
    ) -> Result<Self, PlayError> {
        info!("META_SETMAPPERFLAGS: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_pal_entries(
        self,
        record_number: usize,
        record: META_SETPALENTRIES,
    ) -> Result<Self, PlayError> {
        info!("META_SETPALENTRIES: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_polyfill_mode(
        mut self,
        record_number: usize,
        record: META_SETPOLYFILLMODE,
    ) -> Result<Self, PlayError> {
        self.context_current =
            self.context_current.poly_fill_mode(record.poly_fill_mode);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_relabs(
        self,
        record_number: usize,
        record: META_SETRELABS,
    ) -> Result<Self, PlayError> {
        info!("META_SETRELABS: reserved record and not supported");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_raster_operation(
        mut self,
        record_number: usize,
        record: META_SETROP2,
    ) -> Result<Self, PlayError> {
        self.context_current = self.context_current.draw_mode(record.draw_mode);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_stretch_blt_mode(
        self,
        record_number: usize,
        record: META_SETSTRETCHBLTMODE,
    ) -> Result<Self, PlayError> {
        info!("META_SETSTRETCHBLTMODE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_align(
        mut self,
        record_number: usize,
        record: META_SETTEXTALIGN,
    ) -> Result<Self, PlayError> {
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

        self.context_current = self
            .context_current
            .text_align_update_cp(update_cp)
            .text_align_horizontal(align_horizontal)
            .text_align_vertical(align_vertical);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_char_extra(
        self,
        record_number: usize,
        record: META_SETTEXTCHAREXTRA,
    ) -> Result<Self, PlayError> {
        info!("META_SETTEXTCHAREXTRA: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_color(
        mut self,
        record_number: usize,
        record: META_SETTEXTCOLOR,
    ) -> Result<Self, PlayError> {
        self.context_current =
            self.context_current.text_color(record.color_ref);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_justification(
        self,
        record_number: usize,
        record: META_SETTEXTJUSTIFICATION,
    ) -> Result<Self, PlayError> {
        info!("META_SETTEXTJUSTIFICATION: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_viewport_ext(
        self,
        record_number: usize,
        record: META_SETVIEWPORTEXT,
    ) -> Result<Self, PlayError> {
        info!("META_SETVIEWPORTEXT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_viewport_origin(
        self,
        record_number: usize,
        record: META_SETVIEWPORTORG,
    ) -> Result<Self, PlayError> {
        info!("META_SETVIEWPORTORG: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_window_ext(
        mut self,
        record_number: usize,
        record: META_SETWINDOWEXT,
    ) -> Result<Self, PlayError> {
        self.context_current =
            self.context_current.window_ext(record.x, record.y);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_window_origin(
        mut self,
        record_number: usize,
        record: META_SETWINDOWORG,
    ) -> Result<Self, PlayError> {
        self.context_current =
            self.context_current.window_origin(record.x, record.y);

        Ok(self)
    }

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn escape(
        self,
        record_number: usize,
        record: META_ESCAPE,
    ) -> Result<Self, PlayError> {
        Ok(self)
    }
}
