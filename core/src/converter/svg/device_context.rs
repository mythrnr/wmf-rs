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
    pub layout: Layout,
    pub map_mode: MapMode,
    pub stretch_mode: StretchMode,
    /// Extra inter-character spacing in logical units
    pub text_char_extra: u16,
    /// Number of break characters in the line
    pub text_break_count: u16,
    /// Total extra space in logical units to distribute
    /// across break characters
    pub text_break_extra: u16,
}

impl Default for DeviceContext {
    fn default() -> Self {
        Self {
            object_table: GraphicsObjects::new(0),
            bk_mode: MixMode::TRANSPARENT,
            clipping_region: None,
            drawing_position: PointS { x: 0, y: 0 },
            draw_mode: None,
            layout: Layout::LAYOUT_LTR,
            map_mode: MapMode::MM_TEXT,
            poly_fill_mode: PolyFillMode::ALTERNATE,
            stretch_mode: StretchMode::BLACKONWHITE,
            text_align_horizontal: TextAlignmentMode::TA_LEFT,
            // Per MS-WMF 2.3.5.24, the default text alignment is
            // TA_TOP | TA_LEFT when META_SETTEXTALIGN has not been
            // emitted, so the reference y points to the top edge of
            // the bounding box.
            text_align_vertical: VerticalTextAlignmentMode::VTA_TOP,
            text_align_update_cp: false,
            text_bk_color: ColorRef::white(),
            text_break_count: 0,
            text_break_extra: 0,
            text_char_extra: 0,
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

    pub fn layout(&mut self, layout: Layout) {
        self.layout = layout;
    }

    pub fn map_mode(&mut self, map_mode: MapMode) {
        self.map_mode = map_mode;
        self.window.map_mode = map_mode;
    }

    pub fn poly_fill_mode(&mut self, poly_fill_mode: PolyFillMode) {
        self.poly_fill_mode = poly_fill_mode;
    }

    pub fn stretch_mode(&mut self, stretch_mode: StretchMode) {
        self.stretch_mode = stretch_mode;
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

    pub fn text_char_extra(&mut self, char_extra: u16) {
        self.text_char_extra = char_extra;
    }

    pub fn text_color(&mut self, text_color: ColorRef) {
        self.text_color = text_color;
    }

    pub fn text_justification(&mut self, break_count: u16, break_extra: u16) {
        self.text_break_count = break_count;
        self.text_break_extra = break_extra;
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

    pub fn offset_window_origin(&mut self, x: i16, y: i16) {
        self.window.offset_origin(x, y);
    }

    pub fn viewport_origin(&mut self, x: i16, y: i16) {
        self.window.viewport_origin(x, y);
    }

    pub fn offset_viewport_origin(&mut self, x: i16, y: i16) {
        self.window.offset_viewport_origin(x, y);
    }

    pub fn viewport_ext(&mut self, x: i16, y: i16) {
        self.window.viewport_ext(x, y);
    }

    pub fn scale_viewport_ext(
        &mut self,
        x_num: i16,
        x_denom: i16,
        y_num: i16,
        y_denom: i16,
    ) {
        self.window.scale_viewport_ext(
            f32::from(x_num),
            f32::from(x_denom),
            f32::from(y_num),
            f32::from(y_denom),
        );
    }
}

impl DeviceContext {
    pub fn as_css_text_align(&self) -> String {
        match self.text_align_horizontal {
            TextAlignmentMode::TA_CENTER => "middle".to_owned(),
            TextAlignmentMode::TA_RIGHT => "end".to_owned(),
            _ => "start".to_owned(),
        }
    }

    /// Compute the offset, in SVG units, to add to the WMF reference
    /// y so the resulting SVG `y` lands on the alphabetic baseline.
    ///
    /// `dominant-baseline` is unreliable across SVG renderers; many
    /// fall back to `auto` (alphabetic) regardless of the requested
    /// value, which then offsets the text vertically. Pre-shifting
    /// `y` to the baseline and emitting no `dominant-baseline`
    /// attribute keeps placement consistent across renderers.
    ///
    /// Ascent and descent fractions of the em differ by script, so
    /// the charset selects the ratio: Latin/European (ANSI etc.)
    /// uses ascent ≈ 0.8em and descent ≈ 0.2em; CJK (Shift-JIS,
    /// Big5, GB2312, Hangul, Johab) glyphs fill nearly the whole em
    /// above the baseline, so ascent ≈ 1.0em and descent ≈ 0em.
    ///
    /// Real metrics depend on the font, which the WMF stream does
    /// not embed; these averages are accurate enough to keep text
    /// inside its WMF bounding rectangle for both scripts.
    pub fn text_baseline_y_offset(
        &self,
        font_height: i16,
        charset: CharacterSet,
    ) -> i16 {
        let em = f32::from(font_height.abs());
        let (ascent_ratio, descent_ratio) = match charset {
            CharacterSet::SHIFTJIS_CHARSET
            | CharacterSet::HANGUL_CHARSET
            | CharacterSet::JOHAB_CHARSET
            | CharacterSet::GB2312_CHARSET
            | CharacterSet::CHINESEBIG5_CHARSET => (1.0_f32, 0.0_f32),
            _ => (0.8_f32, 0.2_f32),
        };
        let offset = match self.text_align_vertical {
            // y points to the top edge → shift down by ascent.
            VerticalTextAlignmentMode::VTA_TOP => em * ascent_ratio,
            // y points to the bottom edge → shift up by descent.
            VerticalTextAlignmentMode::VTA_BOTTOM => -em * descent_ratio,
            // y points to the middle of the em box → shift down to
            // reach the baseline (em/2 - descent).
            VerticalTextAlignmentMode::VTA_CENTER => em * (0.5 - descent_ratio),
            // VTA_BASELINE: y already references the baseline.
            _ => 0.0,
        };
        offset.round() as i16
    }

    pub fn point_s_to_absolute_point(&self, point: &PointS) -> PointS {
        // Widen to f32 before subtraction to avoid i16
        // overflow. i16 values fit in f32 without precision
        // loss.
        let lx = f32::from(point.x) - f32::from(self.window.origin_x);
        let ly = f32::from(point.y) - f32::from(self.window.origin_y);
        let (dx, dy) = self.window.logical_to_device(lx, ly);

        // Round to nearest integer instead of truncating to
        // avoid a consistent bias toward negative coordinates.
        PointS { x: dx.round() as i16, y: dy.round() as i16 }
    }

    pub fn point_s_to_relative_point(&self, point: &PointS) -> PointS {
        // Widen to f32 before subtraction to avoid i16
        // overflow. i16 values fit in f32 without precision
        // loss.
        let lx = f32::from(point.x) - f32::from(self.window.origin_x);
        let ly = f32::from(point.y) - f32::from(self.window.origin_y);
        let (dx, dy) = self.window.logical_to_device(lx, ly);

        // Round to nearest integer instead of truncating.
        PointS {
            x: dx.round() as i16 + self.drawing_position.x,
            y: dy.round() as i16 + self.drawing_position.y,
        }
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
    /// Current window extent (set by META_SETWINDOWEXT)
    pub current_ext_x: f32,
    pub current_ext_y: f32,
    /// Viewport origin
    pub viewport_origin_x: f32,
    pub viewport_origin_y: f32,
    /// Viewport extent (None = not explicitly set)
    pub viewport_ext_x: Option<f32>,
    pub viewport_ext_y: Option<f32>,
    /// Current mapping mode
    pub map_mode: MapMode,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            origin_x: 0,
            origin_y: 0,
            scale_x: 1.0,
            scale_y: 1.0,
            min_x: 0,
            min_y: 0,
            flip_x: false,
            flip_y: false,
            current_ext_x: 1.0,
            current_ext_y: 1.0,
            viewport_origin_x: 0.0,
            viewport_origin_y: 0.0,
            viewport_ext_x: None,
            viewport_ext_y: None,
            map_mode: MapMode::MM_TEXT,
        }
    }
}

impl Window {
    /// Returns whether the current mapping mode is a fixed mode
    /// (MM_LOMETRIC through MM_TWIPS). In fixed modes,
    /// META_SETWINDOWEXT / META_SETVIEWPORTEXT do not affect
    /// coordinate conversion.
    fn is_fixed_map_mode(&self) -> bool {
        matches!(
            self.map_mode,
            MapMode::MM_LOMETRIC
                | MapMode::MM_HIMETRIC
                | MapMode::MM_LOENGLISH
                | MapMode::MM_HIENGLISH
                | MapMode::MM_TWIPS
        )
    }
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ext(&mut self, x: i16, y: i16) {
        // In fixed mapping modes, window extent does not affect
        // coordinate conversion (per MS-WMF spec). ViewBox
        // tracking is delegated to extend_window.
        if self.is_fixed_map_mode() {
            return;
        }

        self.flip_x = x < 0;
        self.flip_y = y < 0;

        let mag_x = i16::try_from(x.unsigned_abs()).unwrap_or(i16::MAX);
        let mag_y = i16::try_from(y.unsigned_abs()).unwrap_or(i16::MAX);

        // Preserve any previously tracked larger extents to
        // avoid shrinking the viewBox and clipping already
        // rendered content.
        self.x = self.x.max(mag_x);
        self.y = self.y.max(mag_y);

        // Track window extent for viewport calculations
        self.current_ext_x = f32::from(mag_x);
        self.current_ext_y = f32::from(mag_y);

        // META_SETWINDOWEXT sets an absolute extent, so reset
        // the scale accumulated by META_SCALEWINDOWEXT
        self.scale_x = 1.0;
        self.scale_y = 1.0;
    }

    pub fn origin(&mut self, origin_x: i16, origin_y: i16) {
        self.origin_x = origin_x;
        self.origin_y = origin_y;
    }

    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        // Ignore scale changes in fixed mapping modes
        if self.is_fixed_map_mode() {
            return;
        }

        self.scale_x = scale_x;
        self.scale_y = scale_y;
    }

    pub fn offset_origin(&mut self, x: i16, y: i16) {
        self.origin_x = self.origin_x.saturating_add(x);
        self.origin_y = self.origin_y.saturating_add(y);
    }

    pub fn viewport_origin(&mut self, x: i16, y: i16) {
        self.viewport_origin_x = f32::from(x);
        self.viewport_origin_y = f32::from(y);
    }

    pub fn offset_viewport_origin(&mut self, x: i16, y: i16) {
        self.viewport_origin_x += f32::from(x);
        self.viewport_origin_y += f32::from(y);
    }

    pub fn viewport_ext(&mut self, x: i16, y: i16) {
        // Ignore viewport extent in fixed mapping modes
        if self.is_fixed_map_mode() {
            return;
        }

        self.viewport_ext_x = Some(f32::from(x));
        self.viewport_ext_y = Some(f32::from(y));
    }

    pub fn scale_viewport_ext(
        &mut self,
        x_num: f32,
        x_denom: f32,
        y_num: f32,
        y_denom: f32,
    ) {
        // Ignore viewport extent scaling in fixed mapping modes
        if self.is_fixed_map_mode() {
            return;
        }

        if let Some(ref mut ext_x) = self.viewport_ext_x {
            *ext_x = *ext_x * x_num / x_denom;
        }

        if let Some(ref mut ext_y) = self.viewport_ext_y {
            *ext_y = *ext_y * y_num / y_denom;
        }
    }

    /// Convert logical coordinates to device coordinates.
    /// The conversion method varies depending on the MapMode.
    pub fn logical_to_device(&self, lx: f32, ly: f32) -> (f32, f32) {
        // Fixed mapping modes (MM_LOMETRIC through MM_TWIPS):
        // positive Y points up in logical space, so flip it to
        // match SVG coordinate system (positive Y points down).
        // No scaling by window/viewport extent.
        if self.is_fixed_map_mode() {
            let dx = lx + self.viewport_origin_x;
            let dy = -ly + self.viewport_origin_y;
            return (dx, dy);
        }

        let (dx, dy) = match (self.viewport_ext_x, self.viewport_ext_y) {
            (Some(vp_ext_x), Some(vp_ext_y)) => {
                // When viewport extent is set, apply the WMF
                // formula: Dx = Lx * VEx / WEx
                let eff_win_x = self.current_ext_x * self.scale_x;
                let eff_win_y = self.current_ext_y * self.scale_y;
                let rx = if eff_win_x.abs() > f32::EPSILON {
                    vp_ext_x / eff_win_x
                } else {
                    1.0
                };
                let ry = if eff_win_y.abs() > f32::EPSILON {
                    vp_ext_y / eff_win_y
                } else {
                    1.0
                };

                // MM_ISOTROPIC: preserve aspect ratio by applying
                // the smaller absolute scale to both axes. The
                // direction (sign) of each axis is preserved.
                if self.map_mode == MapMode::MM_ISOTROPIC {
                    let abs_min = rx.abs().min(ry.abs());
                    (lx * abs_min.copysign(rx), ly * abs_min.copysign(ry))
                } else {
                    (lx * rx, ly * ry)
                }
            }
            _ => {
                // Default: preserve existing behavior
                (lx / self.scale_x, ly / self.scale_y)
            }
        };

        // Add viewport origin after axis flip
        let dx = if self.flip_x { -dx } else { dx } + self.viewport_origin_x;
        let dy = if self.flip_y { -dy } else { dy } + self.viewport_origin_y;

        (dx, dy)
    }

    pub fn as_view_box(&self) -> (i32, i32, i32, i32) {
        // Expand viewBox to include negative coordinates if any
        let min_x = i32::from(self.min_x).min(0);
        let min_y = i32::from(self.min_y).min(0);
        let max_x = i32::from(self.x);
        let max_y = i32::from(self.y);

        (min_x, min_y, max_x - min_x, max_y - min_y)
    }
}
