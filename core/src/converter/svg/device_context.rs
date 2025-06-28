use crate::converter::{svg::util::css_color_from_color_ref, *};

#[derive(Clone, Debug)]
pub struct DeviceContext {
    // graphics object
    pub object_table: GraphicsObjects,

    // structures
    pub drawing_position: PointS,
    pub text_bk_color: ColorRef,
    pub text_color: ColorRef,
    pub window: Window,

    // graphics props
    pub bk_mode: MixMode,
    pub clipping_region: Option<Rect>,
    pub poly_fill_mode: PolyFillMode,
    pub text_align_horizontal: TextAlignmentMode,
    pub text_align_vertical: VerticalTextAlignmentMode,
    pub text_align_update_cp: bool,

    pub draw_mode: Option<BinaryRasterOperation>,
    pub map_mode: MapMode,
}

impl Default for DeviceContext {
    fn default() -> Self {
        Self {
            object_table: GraphicsObjects::new(0),
            bk_mode: MixMode::TRANSPARENT,
            clipping_region: None,
            drawing_position: PointS { x: 0, y: 0 },
            draw_mode: None,
            map_mode: MapMode::MM_TEXT,
            poly_fill_mode: PolyFillMode::ALTERNATE,
            text_align_horizontal: TextAlignmentMode::TA_LEFT,
            text_align_vertical: VerticalTextAlignmentMode::VTA_BASELINE,
            text_align_update_cp: false,
            text_bk_color: ColorRef::white(),
            text_color: ColorRef::black(),
            window: Window::new(),
        }
    }
}

impl DeviceContext {
    pub fn create_object_table(mut self, length: u16) -> Self {
        self.object_table = GraphicsObjects::new(length as usize);
        self
    }

    pub fn bk_mode(mut self, bk_mode: MixMode) -> Self {
        self.bk_mode = bk_mode;
        self
    }

    pub fn clipping_region(mut self, clipping_region: Rect) -> Self {
        let clipping_region = if let Some(ref existing) = self.clipping_region {
            if let Some(overlap_region) = existing.overlap(&clipping_region) {
                overlap_region
            } else {
                clipping_region
            }
        } else {
            clipping_region
        };

        self.clipping_region = clipping_region.into();
        self
    }

    pub fn drawing_position(mut self, drawing_position: PointS) -> Self {
        self.drawing_position = drawing_position;
        self
    }

    pub fn draw_mode(mut self, draw_mode: BinaryRasterOperation) -> Self {
        self.draw_mode = draw_mode.into();
        self
    }

    pub fn map_mode(mut self, map_mode: MapMode) -> Self {
        self.map_mode = map_mode;
        self
    }

    pub fn poly_fill_mode(mut self, poly_fill_mode: PolyFillMode) -> Self {
        self.poly_fill_mode = poly_fill_mode;
        self
    }

    pub fn poly_fill_rule(&self) -> String {
        match self.poly_fill_mode {
            PolyFillMode::ALTERNATE => "evenodd",
            PolyFillMode::WINDING => "nonzero",
        }
        .to_owned()
    }

    pub fn text_align_horizontal(
        mut self,
        text_align_horizontal: TextAlignmentMode,
    ) -> Self {
        self.text_align_horizontal = text_align_horizontal;
        self
    }

    pub fn text_align_vertical(
        mut self,
        text_align_vertical: VerticalTextAlignmentMode,
    ) -> Self {
        self.text_align_vertical = text_align_vertical;
        self
    }

    pub fn text_align_update_cp(mut self, text_align_update_cp: bool) -> Self {
        self.text_align_update_cp = text_align_update_cp;
        self
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

    pub fn text_bk_color(mut self, text_bk_color: ColorRef) -> Self {
        self.text_bk_color = text_bk_color;
        self
    }

    pub fn text_color(mut self, text_color: ColorRef) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn text_color_as_css_color(&self) -> String {
        css_color_from_color_ref(&self.text_color)
    }

    pub fn window_ext(mut self, x: i16, y: i16) -> Self {
        self.window = self.window.ext(x, y);
        self
    }

    pub fn window_origin(mut self, x: i16, y: i16) -> Self {
        self.window = self.window.origin(x, y);
        self
    }

    pub fn window_scale(mut self, x: f32, y: f32) -> Self {
        self.window = self.window.scale(x, y);
        self
    }

    pub fn point_s_to_absolute_point(&self, point: &PointS) -> PointS {
        let x = (f32::from((point.x - self.window.origin_x).abs())
            / self.window.scale_x) as i16;
        let y = (f32::from((point.y - self.window.origin_y).abs())
            / self.window.scale_y) as i16;

        PointS { x, y }
    }

    pub fn point_s_to_relative_point(&self, point: &PointS) -> PointS {
        let x = (f32::from((point.x - self.window.origin_x).abs())
            / self.window.scale_x) as i16
            + self.drawing_position.x;
        let y = (f32::from((point.y - self.window.origin_y).abs())
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
        Self::default()
    }

    pub fn ext(mut self, x: i16, y: i16) -> Self {
        self.x = x.abs();
        self.y = y.abs();
        self
    }

    pub fn origin(mut self, origin_x: i16, origin_y: i16) -> Self {
        self.origin_x = origin_x;
        self.origin_y = origin_y;
        self
    }

    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> Self {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }

    pub fn as_view_box(&self) -> (i16, i16, i16, i16) {
        (0, 0, self.x.abs(), self.y.abs())
    }
}
