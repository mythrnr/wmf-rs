use svg::node::element::{path::Data, Image, Path, Pattern};

use crate::parser::*;

pub fn css_color_from_color_ref(c: &ColorRef) -> String {
    format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue)
}

pub fn url_string(link: &str) -> String {
    format!("url({link})")
}

pub fn as_point_string(point: &PointS) -> String {
    format!("{},{}", point.x, point.y)
}

#[derive(Clone, Debug)]
pub enum Fill {
    Pattern { pattern: Pattern },
    Value { value: String },
}

impl From<Brush> for Fill {
    fn from(v: Brush) -> Self {
        match v {
            Brush::DIBPatternPT { brush_hatch, .. } => {
                let data = crate::converter::Bitmap::from(brush_hatch.clone())
                    .as_data_url();
                let image = Image::new()
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", brush_hatch.dib_header_info.width())
                    .set("height", brush_hatch.dib_header_info.height())
                    .set("href", data);
                let pattern = Pattern::new()
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", brush_hatch.dib_header_info.width())
                    .set("height", brush_hatch.dib_header_info.height())
                    .add(image);

                Fill::Pattern { pattern }
            }
            Brush::Hatched { color_ref, brush_hatch } => {
                let path = match brush_hatch {
                    HatchStyle::HS_HORIZONTAL => {
                        let data = Data::new().move_to((0, 0)).line_to((10, 0));

                        Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data)
                    }
                    HatchStyle::HS_VERTICAL => {
                        let data = Data::new().move_to((0, 0)).line_to((0, 10));

                        Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data)
                    }
                    HatchStyle::HS_FDIAGONAL => {
                        let data =
                            Data::new().move_to((0, 10)).line_to((10, 0));

                        Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data)
                    }
                    HatchStyle::HS_BDIAGONAL => {
                        let data =
                            Data::new().move_to((0, 0)).line_to((10, 10));

                        Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data)
                    }
                    HatchStyle::HS_CROSS => {
                        let data = Data::new()
                            .move_to((0, 0))
                            .line_to((10, 0))
                            .move_to((0, 0))
                            .line_to((0, 10));

                        Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data)
                    }
                    HatchStyle::HS_DIAGCROSS => {
                        let data = Data::new()
                            .move_to((0, 0))
                            .line_to((10, 10))
                            .move_to((10, 0))
                            .line_to((0, 10));

                        Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data)
                    }
                };

                let pattern = Pattern::new()
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", 10)
                    .set("height", 10)
                    .add(path);

                Fill::Pattern { pattern }
            }
            Brush::Pattern { brush_hatch } => {
                let data = crate::converter::Bitmap::from(brush_hatch.clone())
                    .as_data_url();
                let image = Image::new()
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", brush_hatch.width)
                    .set("height", brush_hatch.height)
                    .set("href", data);
                let pattern = Pattern::new()
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", brush_hatch.width)
                    .set("height", brush_hatch.height)
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
        let mut stroke =
            Self { color: v.color_ref, width: v.width.x, ..Default::default() };

        for style in v.style {
            stroke = match style {
                PenStyle::PS_DASH => Self {
                    dash_array: format!("{v} {v}", v = stroke.width * 10),
                    ..stroke
                },
                PenStyle::PS_DOT | PenStyle::PS_ALTERNATE => Self {
                    dash_array: format!(
                        "{} {}",
                        stroke.width,
                        stroke.width * 10
                    ),
                    ..stroke
                },
                PenStyle::PS_DASHDOT => Self {
                    dash_array: format!(
                        "{} {} {} {}",
                        stroke.width * 10,
                        stroke.width * 2,
                        stroke.width,
                        stroke.width * 2,
                    ),
                    ..stroke
                },
                PenStyle::PS_DASHDOTDOT => Self {
                    dash_array: format!(
                        "{} {} {} {} {} {}",
                        stroke.width * 10,
                        stroke.width * 2,
                        stroke.width,
                        stroke.width * 2,
                        stroke.width,
                        stroke.width * 2,
                    ),
                    ..stroke
                },
                PenStyle::PS_NULL => Self { opacity: 0_f32, ..stroke },
                PenStyle::PS_ENDCAP_SQUARE => {
                    Self { line_cap: "square".to_owned(), ..stroke }
                }
                PenStyle::PS_JOIN_BEVEL => {
                    Self { line_join: "bevel".to_owned(), ..stroke }
                }
                PenStyle::PS_JOIN_MITER => {
                    Self { line_join: "miter".to_owned(), ..stroke }
                }
                PenStyle::PS_SOLID => stroke,
                // not implemented
                PenStyle::PS_INSIDEFRAME
                | PenStyle::PS_USERSTYLE
                | PenStyle::PS_ENDCAP_FLAT => {
                    tracing::info!(?style, "pen style is not implemented");
                    stroke
                }
            };
        }

        stroke
    }
}

impl From<Brush> for Stroke {
    fn from(v: Brush) -> Self {
        match v {
            Brush::DIBPatternPT { .. } => {
                Self { opacity: 0_f32, ..Default::default() }
            }
            Brush::Hatched { color_ref, .. } | Brush::Solid { color_ref } => {
                Self { color: color_ref, ..Default::default() }
            }
            Brush::Pattern { .. } => Self { ..Default::default() },
            Brush::Null => {
                Self { width: 0, opacity: 0_f32, ..Default::default() }
            }
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

    pub fn set_props<T: svg::Node>(&self, mut elem: T) -> T {
        if self.opacity == 0_f32 {
            elem.assign("stroke", "none");
            return elem;
        }

        elem.assign("stroke", self.color());
        elem.assign("stroke-dasharray", self.dash_array());
        elem.assign("stroke-linecap", self.line_cap());
        elem.assign("stroke-linejoin", self.line_join());
        elem.assign("stroke-opacity", self.opacity());
        elem.assign("stroke-width", self.width());
        elem
    }
}

impl Font {
    pub fn set_props<T: svg::Node>(
        &self,
        mut elem: T,
        point: &PointS,
    ) -> (T, Vec<String>) {
        let mut styles = vec![];

        if self.italic {
            styles.push("font-style: italic;".to_owned());
        }

        {
            let mut v = vec![];

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
            elem.assign("rotate", -self.orientation / 10);
        }

        if self.escapement != 0 {
            elem.assign(
                "transform",
                format!(
                    "rotate({}, {} {})",
                    -self.escapement / 10,
                    point.x,
                    point.y
                ),
            );
        }

        elem.assign("font-family", self.facename.as_str());
        elem.assign("font-size", self.height.abs());
        elem.assign("font-weight", self.weight);

        (elem, styles)
    }
}
