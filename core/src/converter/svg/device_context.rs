use crate::{
    converter::{svg::util::css_color_from_color_ref, *},
    parser::*,
};

#[derive(Clone, Debug)]
pub struct DeviceContext {
    // graphics object
    pub object_table: GraphicsObjects,

    // structures
    pub drawing_position: PointS,
    pub _text_bk_color: ColorRef,
    pub text_color: ColorRef,
    pub window: Window,

    // graphics props
    pub _bk_mode: MixMode,
    pub clipping_region: Option<Rect>,
    pub poly_fill_mode: PolyFillMode,
    pub text_align_horizontal: TextAlignmentMode,
    pub text_align_vertical: VerticalTextAlignmentMode,
    pub text_align_update_cp: bool,

    pub _draw_mode: Option<BinaryRasterOperation>,
    pub _map_mode: MapMode,
}

impl Default for DeviceContext {
    fn default() -> Self {
        Self {
            object_table: GraphicsObjects::new(0),
            _bk_mode: MixMode::TRANSPARENT,
            clipping_region: None,
            drawing_position: PointS { x: 0, y: 0 },
            _draw_mode: None,
            _map_mode: MapMode::MM_TEXT,
            poly_fill_mode: PolyFillMode::ALTERNATE,
            text_align_horizontal: TextAlignmentMode::TA_LEFT,
            text_align_vertical: VerticalTextAlignmentMode::VTA_BASELINE,
            text_align_update_cp: false,
            _text_bk_color: ColorRef::white(),
            text_color: ColorRef::black(),
            window: Window::new(),
        }
    }
}

impl DeviceContext {
    pub fn create_object_table(self, length: u16) -> Self {
        Self { object_table: GraphicsObjects::new(length as usize), ..self }
    }

    pub fn bk_mode(self, bk_mode: MixMode) -> Self {
        Self { _bk_mode: bk_mode.into(), ..self }
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

    pub fn drawing_position(self, drawing_position: PointS) -> Self {
        Self { drawing_position, ..self }
    }

    pub fn draw_mode(self, draw_mode: BinaryRasterOperation) -> Self {
        Self { _draw_mode: draw_mode.into(), ..self }
    }

    pub fn map_mode(self, map_mode: MapMode) -> Self {
        Self { _map_mode: map_mode, ..self }
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

    pub fn text_align_horizontal(
        self,
        text_align_horizontal: TextAlignmentMode,
    ) -> Self {
        Self { text_align_horizontal, ..self }
    }

    pub fn text_align_vertical(
        self,
        text_align_vertical: VerticalTextAlignmentMode,
    ) -> Self {
        Self { text_align_vertical, ..self }
    }

    pub fn text_align_update_cp(self, text_align_update_cp: bool) -> Self {
        Self { text_align_update_cp, ..self }
    }

    pub fn as_css_text_align(&self) -> String {
        match self.text_align_horizontal {
            TextAlignmentMode::TA_CENTER => "middle".to_owned(),
            TextAlignmentMode::TA_RIGHT => "end".to_owned(),
            _ => "start".to_owned(),
        }
    }

    pub fn as_css_text_align_vertical(&self) -> String {
        match self.text_align_vertical {
            VerticalTextAlignmentMode::VTA_BOTTOM => "text-bottom".to_owned(),
            VerticalTextAlignmentMode::VTA_TOP => "text-top".to_owned(),
            VerticalTextAlignmentMode::VTA_CENTER => "central".to_owned(),
            _ => "auto".to_owned(),
        }
    }

    pub fn text_bk_color(self, text_bk_color: ColorRef) -> Self {
        Self { _text_bk_color: text_bk_color, ..self }
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

    pub fn point_s_to_absolute_point(&self, point: &PointS) -> PointS {
        let x = ((point.x - self.window.origin_x).abs() as f32
            / self.window.scale_x) as i16;
        let y = ((point.y - self.window.origin_y).abs() as f32
            / self.window.scale_y) as i16;

        PointS { x, y }
    }

    pub fn point_s_to_relative_point(&self, point: &PointS) -> PointS {
        let x = ((point.x - self.window.origin_x).abs() as f32
            / self.window.scale_x) as i16
            + self.drawing_position.x;
        let y = ((point.y - self.window.origin_y).abs() as f32
            / self.window.scale_y) as i16
            + self.drawing_position.y;

        PointS { x, y }
    }

    pub fn extend_window(self, p: &PointS) -> Self {
        let (mut x, mut y) = (0, 0);

        if self.window.x < p.x {
            x = p.x;
        }

        if self.window.y < p.y {
            y = p.y;
        }

        if x > 0 && y > 0 {
            self.window_ext(x, y)
        } else {
            self
        }
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
