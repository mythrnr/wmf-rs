use std::collections::BTreeSet;

use wmf_core::*;

use crate::svg::css_color_from_color_ref;

#[derive(Clone, Debug)]
pub struct DeviceContext {
    pub bk_color: ColorRef,
    pub bk_mode: Option<MixMode>,
    pub brush: Brush,
    pub clipping_region: Option<Rect>,
    pub current_coordinate: Coordinate,
    pub draw_mode: Option<BinaryRasterOperation>,
    pub font: Font,
    pub map_mode: MapMode,
    pub pen: Pen,
    pub poly_fill_mode: PolyFillMode,
    // TextAlignmentMode or VerticalTextAlignmentMode
    pub text_align: BTreeSet<u16>,
    pub text_color: ColorRef,
    pub window: Window,
}

impl Default for DeviceContext {
    fn default() -> Self {
        Self {
            bk_color: ColorRef::white(),
            bk_mode: None,
            brush: Brush::Null,
            clipping_region: None,
            current_coordinate: Coordinate::new(0, 0),
            draw_mode: None,
            font: Font {
                height: 12,
                width: 12,
                escapement: 0,
                orientation: 0,
                weight: 0,
                italic: false,
                underline: false,
                strike_out: false,
                charset: CharacterSet::ANSI_CHARSET,
                out_precision: OutPrecision::OUT_DEFAULT_PRECIS,
                clip_precision: ClipPrecision::CLIP_DEFAULT_PRECIS,
                quality: FontQuality::DEFAULT_QUALITY,
                pitch_and_family: PitchAndFamily {
                    family: FamilyFont::FF_DONTCARE,
                    pitch: PitchFont::DEFAULT_PITCH,
                },
                facename: "System".to_owned(),
            },
            map_mode: MapMode::MM_TEXT,
            pen: Pen {
                style: PenStyle::PS_SOLID,
                width: PointS { x: 1, y: 1 },
                color_ref: ColorRef::black(),
            },
            poly_fill_mode: PolyFillMode::ALTERNATE,
            text_align: BTreeSet::new(),
            text_color: ColorRef::black(),
            window: Window::new(),
        }
    }
}

impl DeviceContext {
    pub fn bk_color(self, bk_color: ColorRef) -> Self {
        Self { bk_color, ..self }
    }

    pub fn bk_color_as_css_color(&self) -> String {
        css_color_from_color_ref(&self.bk_color)
    }

    pub fn bk_mode(self, bk_mode: MixMode) -> Self {
        Self { bk_mode: bk_mode.into(), ..self }
    }

    pub fn clipping_region(self, clipping_region: Rect) -> Self {
        let clipping_region = if let Some(ref existing) = self.clipping_region {
            if let Some(overlap_region) = existing.overlap(&clipping_region) {
                overlap_region
            } else {
                clipping_region
            }
        } else {
            clipping_region
        };

        Self { clipping_region: clipping_region.into(), ..self }
    }

    pub fn current_coordinate(self, current_coordinate: Coordinate) -> Self {
        Self { current_coordinate, ..self }
    }

    pub fn draw_mode(self, draw_mode: BinaryRasterOperation) -> Self {
        Self { draw_mode: draw_mode.into(), ..self }
    }

    pub fn map_mode(self, map_mode: MapMode) -> Self {
        Self { map_mode, ..self }
    }

    pub fn poly_fill_mode(self, poly_fill_mode: PolyFillMode) -> Self {
        Self { poly_fill_mode, ..self }
    }

    pub fn poly_fill_rule(&self) -> String {
        match self.poly_fill_mode {
            PolyFillMode::ALTERNATE => "evenodd",
            PolyFillMode::WINDING => "nonzero",
        }
        .to_owned()
    }

    pub fn text_align(self, text_align: BTreeSet<u16>) -> Self {
        Self { text_align, ..self }
    }

    pub fn text_horizon_align(&self) -> String {
        if self.text_align.contains(&(TextAlignmentMode::TA_CENTER as u16)) {
            return "middle".to_owned();
        }

        if self.text_align.contains(&(TextAlignmentMode::TA_RIGHT as u16)) {
            return "end".to_owned();
        }

        "start".to_owned()
    }

    pub fn text_color(self, text_color: ColorRef) -> Self {
        Self { text_color, ..self }
    }

    pub fn text_color_as_css_color(&self) -> String {
        css_color_from_color_ref(&self.text_color)
    }

    pub fn window_ext(self, x: i16, y: i16) -> Self {
        Self { window: self.window.ext(x, y), ..self }
    }

    pub fn window_origin(self, x: i16, y: i16) -> Self {
        Self { window: self.window.origin(x, y), ..self }
    }

    pub fn window_scale(self, x: f32, y: f32) -> Self {
        Self { window: self.window.scale(x, y), ..self }
    }

    pub fn point_s_to_absolute_point(&self, point: &PointS) -> Coordinate {
        let x = ((point.x - self.window.origin_x).abs() as f32
            / self.window.scale_x) as i16;
        let y = ((point.y - self.window.origin_y).abs() as f32
            / self.window.scale_y) as i16;

        Coordinate { x, y }
    }

    pub fn point_s_to_relative_point(&self, point: &PointS) -> Coordinate {
        let x = ((point.x - self.window.origin_x).abs() as f32
            / self.window.scale_x) as i16
            + self.current_coordinate.x;
        let y = ((point.y - self.window.origin_y).abs() as f32
            / self.window.scale_y) as i16
            + self.current_coordinate.y;

        Coordinate { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl Coordinate {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn as_point_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

#[derive(Clone, Debug)]
pub struct Window {
    pub x: i16,
    pub y: i16,
    pub origin_x: i16,
    pub origin_y: i16,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            x: 1024,
            y: 1024,
            origin_x: 0,
            origin_y: 0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ext(self, x: i16, y: i16) -> Self {
        Self { x: x.abs(), y: y.abs(), ..self }
    }

    pub fn origin(self, origin_x: i16, origin_y: i16) -> Self {
        let x = self.x - self.origin_x + origin_x;
        let y = self.y - self.origin_y + origin_y;

        Self { x, y, origin_x, origin_y, ..self }
    }

    pub fn scale(self, scale_x: f32, scale_y: f32) -> Self {
        Self { scale_x, scale_y, ..self }
    }

    pub fn as_view_box(&self) -> (i16, i16, i16, i16) {
        (0, 0, self.x.abs(), self.y.abs())
    }
}
