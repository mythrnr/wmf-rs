use crate::{
    converter::svg::node::{Data, Node},
    imports::*,
    parser::*,
};

pub fn css_color_from_color_ref(c: &ColorRef) -> String {
    format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue)
}

pub fn url_string(link: &str) -> String {
    format!("url({link})")
}

pub fn as_point_string(point: &PointS) -> String {
    format!("{},{}", point.x, point.y)
}

impl crate::converter::Bitmap {
    pub fn as_data_url(&self) -> String {
        use base64::{Engine, engine::general_purpose::STANDARD};
        format!("data:image/bmp;base64,{}", STANDARD.encode(self.as_slice()))
    }
}

#[derive(Clone, Debug)]
pub enum Fill {
    Pattern { pattern: Node },
    Value { value: String },
}

impl From<&Brush> for Fill {
    fn from(v: &Brush) -> Self {
        match v {
            Brush::DIBPatternPT { brush_hatch, .. } => {
                let data = crate::converter::Bitmap::from(brush_hatch.clone())
                    .as_data_url();
                let image = Node::new("image")
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", brush_hatch.dib_header_info.width())
                    .set("height", brush_hatch.dib_header_info.height())
                    .set("href", data);
                let pattern = Node::new("pattern")
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", brush_hatch.dib_header_info.width())
                    .set("height", brush_hatch.dib_header_info.height())
                    .add(image);

                Fill::Pattern { pattern }
            }
            Brush::Hatched { color_ref, brush_hatch } => {
                let path = match brush_hatch {
                    HatchStyle::HS_HORIZONTAL => {
                        let data = Data::new().move_to("0 0").line_to("10 0");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(color_ref))
                            .set("d", data)
                    }
                    HatchStyle::HS_VERTICAL => {
                        let data = Data::new().move_to("0 0").line_to("0 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(color_ref))
                            .set("d", data)
                    }
                    HatchStyle::HS_FDIAGONAL => {
                        let data = Data::new().move_to("0 10").line_to("10 0");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(color_ref))
                            .set("d", data)
                    }
                    HatchStyle::HS_BDIAGONAL => {
                        let data = Data::new().move_to("0 0").line_to("10 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(color_ref))
                            .set("d", data)
                    }
                    HatchStyle::HS_CROSS => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to("10 0")
                            .move_to("0 0")
                            .line_to("0 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(color_ref))
                            .set("d", data)
                    }
                    HatchStyle::HS_DIAGCROSS => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to("10 10")
                            .move_to("10 0")
                            .line_to("0 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(color_ref))
                            .set("d", data)
                    }
                };

                let pattern = Node::new("pattern")
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", "10")
                    .set("height", "10")
                    .add(path);

                Fill::Pattern { pattern }
            }
            Brush::Pattern { brush_hatch } => {
                let bitmap = crate::parser::DeviceIndependentBitmap::from(
                    brush_hatch.clone(),
                );
                let data = crate::converter::Bitmap::from(bitmap).as_data_url();
                let image = Node::new("image")
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", brush_hatch.width)
                    .set("height", brush_hatch.height)
                    .set("href", data);
                let pattern = Node::new("pattern")
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", brush_hatch.width)
                    .set("height", brush_hatch.height)
                    .add(image);

                Fill::Pattern { pattern }
            }
            Brush::Solid { color_ref } => {
                Fill::Value { value: css_color_from_color_ref(color_ref) }
            }
            Brush::Null => Fill::Value { value: "none".to_owned() },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stroke {
    /// set true if stroke should not be rendered
    none: bool,
    /// sets the color of the line around an element
    color: ColorRef,
    /// sets the width of the line around an element
    width: i16,
    /// sets the opacity of the line around an element
    opacity: f32,
    /// sets the shape of the end-lines for a line or open path
    line_cap: String,
    /// sets the line to show as a dashed line
    dash_array: String,
    /// sets the shape of the corners where two lines meet
    line_join: String,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            none: false,
            color: ColorRef::black(),
            width: 1,
            opacity: 1_f32,
            line_cap: "butt".to_owned(),
            dash_array: "none".to_owned(),
            line_join: "miter".to_owned(),
        }
    }
}

impl From<&Pen> for Stroke {
    fn from(v: &Pen) -> Self {
        if v.style.style == PenStyle::PS_NULL {
            return Self { none: true, ..Default::default() };
        }

        let mut stroke = Self::default();

        match v.style.end_cap {
            PenStyle::PS_SOLID => {
                "round".clone_into(&mut stroke.line_cap);
            }
            PenStyle::PS_ENDCAP_SQUARE => {
                "square".clone_into(&mut stroke.line_cap);
            }
            _ => {
                "butt".clone_into(&mut stroke.line_cap);
            }
        }

        match v.style.line_join {
            PenStyle::PS_SOLID => {
                "round".clone_into(&mut stroke.line_join);
            }
            PenStyle::PS_JOIN_BEVEL => {
                "bevel".clone_into(&mut stroke.line_join);
            }
            _ => {
                "miter".clone_into(&mut stroke.line_join);
            }
        }

        // Dash pattern ratios relative to pen width, based on
        // typical Windows GDI behavior:
        //   dash = 4w, dot = w, gap = 2w
        // Use at least 1 for the effective width so that
        // cosmetic pens (width 0) produce valid patterns.
        let w = i16::max(v.width.x, 1);

        match v.style.style {
            PenStyle::PS_DASH => {
                let dash = w.saturating_mul(4);
                let gap = w.saturating_mul(2);
                stroke.dash_array = format!("{dash} {gap}");
            }
            PenStyle::PS_DOT => {
                let dot = w;
                let gap = w.saturating_mul(2);
                stroke.dash_array = format!("{dot} {gap}");
            }
            PenStyle::PS_ALTERNATE => {
                "1 1".clone_into(&mut stroke.dash_array);
            }
            PenStyle::PS_DASHDOT => {
                let dash = w.saturating_mul(4);
                let dot = w;
                let gap = w.saturating_mul(2);
                stroke.dash_array = format!("{dash} {gap} {dot} {gap}");
            }
            PenStyle::PS_DASHDOTDOT => {
                let dash = w.saturating_mul(4);
                let dot = w;
                let gap = w.saturating_mul(2);
                stroke.dash_array =
                    format!("{dash} {gap} {dot} {gap} {dot} {gap}");
            }
            _ => {}
        }

        stroke.color = v.color_ref.clone();
        stroke.width = if v.width.x == 0 { 1 } else { v.width.x };
        stroke
    }
}

impl Stroke {
    pub fn set_props(&self, elem: Node) -> Node {
        if self.none {
            return elem.set("stroke", "none");
        }

        elem.set("stroke", css_color_from_color_ref(&self.color))
            .set("stroke-dasharray", &self.dash_array)
            .set("stroke-linecap", &self.line_cap)
            .set("stroke-linejoin", &self.line_join)
            .set("stroke-opacity", format!("{:.02}", self.opacity))
            .set("stroke-width", self.width)
    }
}

impl Font {
    pub fn set_props(
        &self,
        mut elem: Node,
        point: &PointS,
    ) -> (Node, Vec<String>) {
        let mut styles = Vec::with_capacity(2);

        if self.italic {
            styles.push("font-style: italic;".to_owned());
        }

        {
            let mut v = Vec::with_capacity(2);

            if self.underline {
                v.push("underline");
            }

            if self.strike_out {
                v.push("line-through");
            }

            if !v.is_empty() {
                styles.push(format!("text-decoration: {};", v.join(" ")));
            }
        };

        // WMF orientation and escapement are in tenths of
        // degrees. Convert to degrees using f32 to preserve
        // sub-degree precision and avoid i16 overflow on
        // subtraction.
        if self.orientation != 0 {
            let ori_deg = (f32::from(self.orientation)
                - f32::from(self.escapement))
                / 10.0;

            if ori_deg.abs() > f32::EPSILON {
                elem = elem.set("rotate", -ori_deg);
            }
        }

        if self.escapement != 0 {
            let esc_deg = f32::from(self.escapement) / 10.0;

            elem = elem.set(
                "transform",
                format!("rotate({}, {} {})", -esc_deg, point.x, point.y,),
            );
        }

        let mut font_family: Vec<&str> = vec![];

        font_family.push(self.facename.as_str());
        self.fallback_facename.iter().for_each(|f| {
            font_family.push(f.as_str());
        });

        elem = elem
            .set("font-family", format!("'{}'", font_family.join("','")))
            .set("font-size", self.height.abs())
            .set("font-weight", self.weight);

        (elem, styles)
    }
}
