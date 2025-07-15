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
        use base64::{engine::general_purpose::STANDARD, Engine};
        format!("data:image/bmp;base64,{}", STANDARD.encode(self.as_slice()))
    }
}

impl Brush {
    pub fn as_filter(&self) -> Node {
        match self {
            Brush::DIBPatternPT { brush_hatch, .. } => {
                let data = crate::converter::Bitmap::from(brush_hatch.clone())
                    .as_data_url();
                Node::new("filter").add(Node::new("feImage").set("href", data))
            }
            Brush::Hatched { color_ref, brush_hatch } => {
                let data = crate::converter::Bitmap::from((
                    color_ref.clone(),
                    *brush_hatch,
                ))
                .as_data_url();
                Node::new("filter").add(Node::new("feImage").set("href", data))
            }
            Brush::Pattern { brush_hatch } => {
                let bitmap = crate::parser::DeviceIndependentBitmap::from(
                    brush_hatch.clone(),
                );
                let data = crate::converter::Bitmap::from(bitmap).as_data_url();

                Node::new("filter").add(Node::new("feImage").set("href", data))
            }
            Brush::Solid { color_ref } => Node::new("filter")
                .set("x", "0")
                .set("y", "0")
                .set("width", "1")
                .set("height", "1")
                .add(
                    Node::new("feFlood")
                        .set("flood-color", css_color_from_color_ref(color_ref))
                        .set("result", "bg"),
                )
                .add(
                    Node::new("feMerge")
                        .add(Node::new("feMergeNode").set("in", "bg"))
                        .add(
                            Node::new("feMergeNode").set("in", "SourceGraphic"),
                        ),
                ),
            Brush::Null => Node::new("filter")
                .set("x", "0")
                .set("y", "0")
                .set("width", "0")
                .set("height", "0")
                .add(
                    Node::new("feFlood")
                        .set("flood-color", "#000")
                        .set("flood-opacity", "0"),
                ),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Fill {
    Pattern { pattern: Node },
    Value { value: String },
}

impl From<Brush> for Fill {
    fn from(v: Brush) -> Self {
        match v {
            Brush::DIBPatternPT { brush_hatch, .. } => {
                let data = crate::converter::Bitmap::from(brush_hatch.clone())
                    .as_data_url();
                let image = Node::new("image")
                    .set("x", "0")
                    .set("y", "0")
                    .set(
                        "width",
                        brush_hatch.dib_header_info.width().to_string(),
                    )
                    .set(
                        "height",
                        brush_hatch.dib_header_info.height().to_string(),
                    )
                    .set("href", data);
                let pattern = Node::new("pattern")
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", "0")
                    .set("y", "0")
                    .set(
                        "width",
                        brush_hatch.dib_header_info.width().to_string(),
                    )
                    .set(
                        "height",
                        brush_hatch.dib_header_info.height().to_string(),
                    )
                    .add(image);

                Fill::Pattern { pattern }
            }
            Brush::Hatched { color_ref, brush_hatch } => {
                let path = match brush_hatch {
                    HatchStyle::HS_HORIZONTAL => {
                        let data = Data::new().move_to("0 0").line_to("10 0");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("d", data.to_string())
                    }
                    HatchStyle::HS_VERTICAL => {
                        let data = Data::new().move_to("0 0").line_to("0 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("d", data.to_string())
                    }
                    HatchStyle::HS_FDIAGONAL => {
                        let data = Data::new().move_to("0 10").line_to("10 0");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("d", data.to_string())
                    }
                    HatchStyle::HS_BDIAGONAL => {
                        let data = Data::new().move_to("0 0").line_to("10 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("d", data.to_string())
                    }
                    HatchStyle::HS_CROSS => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to("10 0")
                            .move_to("0 0")
                            .line_to("0 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("d", data.to_string())
                    }
                    HatchStyle::HS_DIAGCROSS => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to("10 10")
                            .move_to("10 0")
                            .line_to("0 10");

                        Node::new("path")
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("d", data.to_string())
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
                    .set("width", brush_hatch.width.to_string())
                    .set("height", brush_hatch.height.to_string())
                    .set("href", data);
                let pattern = Node::new("pattern")
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", brush_hatch.width.to_string())
                    .set("height", brush_hatch.height.to_string())
                    .add(image);

                Fill::Pattern { pattern }
            }
            Brush::Solid { color_ref } => {
                Fill::Value { value: css_color_from_color_ref(&color_ref) }
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

impl From<Pen> for Stroke {
    fn from(v: Pen) -> Self {
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

        match v.style.style {
            PenStyle::PS_DASH => {
                stroke.dash_array = format!("{v} {v}", v = v.width.x * 10);
            }
            PenStyle::PS_ALTERNATE | PenStyle::PS_DOT => {
                stroke.dash_array = format!("{} {}", v.width.x, v.width.x * 10);
            }
            PenStyle::PS_DASHDOT => {
                stroke.dash_array = format!(
                    "{} {} {} {}",
                    v.width.x * 10,
                    v.width.x * 2,
                    v.width.x,
                    v.width.x * 2,
                );
            }
            PenStyle::PS_DASHDOTDOT => {
                stroke.dash_array = format!(
                    "{} {} {} {} {} {}",
                    v.width.x * 10,
                    v.width.x * 2,
                    v.width.x,
                    v.width.x * 2,
                    v.width.x,
                    v.width.x * 2,
                );
            }
            _ => {}
        }

        stroke.color = v.color_ref;
        stroke.width = if v.width.x == 0 { 1 } else { v.width.x };
        stroke
    }
}

impl From<Brush> for Stroke {
    fn from(v: Brush) -> Self {
        match v {
            Brush::DIBPatternPT { .. } => {
                Self { none: true, ..Default::default() }
            }
            Brush::Hatched { color_ref, .. } | Brush::Solid { color_ref } => {
                Self { color: color_ref, ..Default::default() }
            }
            Brush::Pattern { .. } => Self { ..Default::default() },
            Brush::Null => Self { none: true, width: 0, ..Default::default() },
        }
    }
}

impl Stroke {
    pub fn color(&self) -> String {
        css_color_from_color_ref(&self.color)
    }

    pub fn dash_array(&self) -> String {
        self.dash_array.clone()
    }

    pub fn line_cap(&self) -> String {
        self.line_cap.clone()
    }

    pub fn line_join(&self) -> String {
        self.line_join.clone()
    }

    pub fn opacity(&self) -> String {
        format!("{:.02}", self.opacity)
    }

    pub fn width(&self) -> i16 {
        self.width
    }

    pub fn set_props(&self, elem: Node) -> Node {
        if self.none {
            return elem.set("stroke", "none");
        }

        elem.set("stroke", self.color())
            .set("stroke-dasharray", self.dash_array())
            .set("stroke-linecap", self.line_cap())
            .set("stroke-linejoin", self.line_join())
            .set("stroke-opacity", self.opacity())
            .set("stroke-width", self.width().to_string())
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

        if self.orientation != 0 {
            let ori = self.orientation - self.escapement;

            if ori != 0 {
                elem = elem.set("rotate", (-ori / 10).to_string());
            }
        }

        if self.escapement != 0 {
            elem = elem.set(
                "transform",
                format!(
                    "rotate({}, {} {})",
                    -self.escapement / 10,
                    point.x,
                    point.y
                ),
            );
        }

        elem = elem
            .set("font-family", self.facename.as_str())
            .set("font-size", self.height.abs().to_string())
            .set("font-weight", self.weight.to_string());

        (elem, styles)
    }
}
