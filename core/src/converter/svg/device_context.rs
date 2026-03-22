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

// mutations
impl DeviceContext {
    pub fn bk_mode(&mut self, bk_mode: MixMode) {
        self.bk_mode = bk_mode;
    }

    pub fn create_object_table(&mut self, length: u16) {
        self.object_table = GraphicsObjects::new(length as usize);
    }

    pub fn clipping_region(&mut self, clipping_region: Rect) {
        self.clipping_region = if let Some(ref existing) = self.clipping_region
        {
            existing.overlap(&clipping_region).unwrap_or(clipping_region)
        } else {
            clipping_region
        }
        .into();
    }

    pub fn drawing_position(&mut self, drawing_position: PointS) {
        self.drawing_position = drawing_position;
    }

    pub fn draw_mode(&mut self, draw_mode: BinaryRasterOperation) {
        self.draw_mode = draw_mode.into();
    }

    pub fn extend_window(&mut self, p: &PointS) {
        // Track minimum coordinates for viewBox expansion
        self.window.min_x = self.window.min_x.min(p.x);
        self.window.min_y = self.window.min_y.min(p.y);

        // Track maximum coordinates (extend x and y independently)
        if self.window.x < p.x {
            self.window.x = p.x;
        }

        if self.window.y < p.y {
            self.window.y = p.y;
        }
    }

    pub fn map_mode(&mut self, map_mode: MapMode) {
        self.map_mode = map_mode;
    }

    pub fn poly_fill_mode(&mut self, poly_fill_mode: PolyFillMode) {
        self.poly_fill_mode = poly_fill_mode;
    }

    pub fn text_align_horizontal(
        &mut self,
        text_align_horizontal: TextAlignmentMode,
    ) {
        self.text_align_horizontal = text_align_horizontal;
    }

    pub fn text_align_vertical(
        &mut self,
        text_align_vertical: VerticalTextAlignmentMode,
    ) {
        self.text_align_vertical = text_align_vertical;
    }

    pub fn text_align_update_cp(&mut self, text_align_update_cp: bool) {
        self.text_align_update_cp = text_align_update_cp;
    }

    pub fn text_bk_color(&mut self, text_bk_color: ColorRef) {
        self.text_bk_color = text_bk_color;
    }

    pub fn text_color(&mut self, text_color: ColorRef) {
        self.text_color = text_color;
    }

    pub fn window_ext(&mut self, x: i16, y: i16) {
        self.window.ext(x, y);
    }

    pub fn window_origin(&mut self, x: i16, y: i16) {
        self.window.origin(x, y);
    }

    pub fn window_scale(&mut self, x: f32, y: f32) {
        self.window.scale(x, y);
    }
}

impl DeviceContext {
    pub fn text_y_offset(&self, font_height: i16) -> i16 {
        if matches!(
            self.text_align_vertical,
            VerticalTextAlignmentMode::VTA_BASELINE
                | VerticalTextAlignmentMode::VTA_BOTTOM
        ) && font_height < 0
        {
            -font_height
        } else {
            0
        }
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

    pub fn point_s_to_absolute_point(&self, point: &PointS) -> PointS {
        let dx =
            f32::from(point.x - self.window.origin_x) / self.window.scale_x;
        let dy =
            f32::from(point.y - self.window.origin_y) / self.window.scale_y;

        // Negative extent means the axis is flipped
        let x = (if self.window.flip_x { -dx } else { dx }) as i16;
        let y = (if self.window.flip_y { -dy } else { dy }) as i16;

        PointS { x, y }
    }

    pub fn point_s_to_relative_point(&self, point: &PointS) -> PointS {
        let dx =
            f32::from(point.x - self.window.origin_x) / self.window.scale_x;
        let dy =
            f32::from(point.y - self.window.origin_y) / self.window.scale_y;

        let x = (if self.window.flip_x { -dx } else { dx }) as i16
            + self.drawing_position.x;
        let y = (if self.window.flip_y { -dy } else { dy }) as i16
            + self.drawing_position.y;

        PointS { x, y }
    }

    pub fn poly_fill_rule(&self) -> &'static str {
        match self.poly_fill_mode {
            PolyFillMode::ALTERNATE => "evenodd",
            PolyFillMode::WINDING => "nonzero",
        }
    }

    pub fn text_color_as_css_color(&self) -> String {
        css_color_from_color_ref(&self.text_color)
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
    /// Minimum rendered coordinates for viewBox expansion
    pub min_x: i16,
    pub min_y: i16,
    /// Flip the axis when the extent is negative
    /// (in WMF, a negative extent reverses the axis direction)
    pub flip_x: bool,
    pub flip_y: bool,
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
            min_x: 0,
            min_y: 0,
            flip_x: false,
            flip_y: false,
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ext(&mut self, x: i16, y: i16) {
        self.flip_x = x < 0;
        self.flip_y = y < 0;
        self.x = i16::try_from(x.unsigned_abs())
            .unwrap_or(i16::MAX);
        self.y = i16::try_from(y.unsigned_abs())
            .unwrap_or(i16::MAX);
    }

    pub fn origin(&mut self, origin_x: i16, origin_y: i16) {
        self.origin_x = origin_x;
        self.origin_y = origin_y;
    }

    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
    }

    pub fn as_view_box(&self) -> (i32, i32, i32, i32) {
        // Expand viewBox to include negative coordinates if any
        let min_x = i32::from(self.min_x).min(0);
        let min_y = i32::from(self.min_y).min(0);

        (
            min_x,
            min_y,
            i32::from(self.x) - min_x,
            i32::from(self.y) - min_y,
        )
    }
}
