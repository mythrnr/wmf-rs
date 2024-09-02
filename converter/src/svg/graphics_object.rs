use svg::node::element::{path::Data, Path, Pattern};
use wmf_core::*;

pub fn random_string() -> String {
    use rand::distributions::{Alphanumeric, DistString};

    let mut rng = rand::thread_rng();

    Alphanumeric.sample_string(&mut rng, 16)
}

pub fn css_color_from_color_ref(c: &ColorRef) -> String {
    format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue)
}

pub fn url_string(link: String) -> String {
    format!("url({link})")
}

#[derive(Clone, Debug)]
pub enum Fill {
    Pattern { pattern: Pattern },
    Value { value: String },
}

impl From<Brush> for Fill {
    fn from(v: Brush) -> Self {
        match v {
            Brush::DIBPatternPT { brush_hatch, .. } => Fill::Value {
                value: url_string(
                    crate::Bitmap::from(brush_hatch).as_data_url(),
                ),
            },
            Brush::Hatched { color_ref, brush_hatch } => {
                let path = match brush_hatch {
                    HatchStyle::HS_HORIZONTAL => {
                        let data = Data::new().move_to((0, 0)).line_to((10, 0));
                        let path = Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data);

                        path
                    }
                    HatchStyle::HS_VERTICAL => {
                        let data = Data::new().move_to((0, 0)).line_to((0, 10));
                        let path = Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data);

                        path
                    }
                    HatchStyle::HS_FDIAGONAL => {
                        let data =
                            Data::new().move_to((0, 10)).line_to((10, 0));
                        let path = Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data);

                        path
                    }
                    HatchStyle::HS_BDIAGONAL => {
                        let data =
                            Data::new().move_to((0, 0)).line_to((10, 10));
                        let path = Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data);

                        path
                    }
                    HatchStyle::HS_CROSS => {
                        let data = Data::new()
                            .move_to((0, 0))
                            .line_to((10, 0))
                            .move_to((0, 0))
                            .line_to((0, 10));
                        let path = Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data);

                        path
                    }
                    HatchStyle::HS_DIAGCROSS => {
                        let data = Data::new()
                            .move_to((0, 0))
                            .line_to((10, 10))
                            .move_to((10, 0))
                            .line_to((0, 10));
                        let path = Path::new()
                            .set("stroke", css_color_from_color_ref(&color_ref))
                            .set("data", data);

                        path
                    }
                };

                let pattern = Pattern::new()
                    .set("patternUnits", "userSpaceOnUse")
                    .set("viewBox", "0 0 10 10")
                    .set("width", 10)
                    .set("height", 10)
                    .add(path);

                Fill::Pattern { pattern }
            }
            Brush::Pattern { brush_hatch } => Fill::Value {
                value: url_string(
                    crate::Bitmap::from(brush_hatch.clone()).as_data_url(),
                ),
            },
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
        match v.style {
            PenStyle::PS_DASH => Self {
                color: v.color_ref,
                dash_array: "6,2".to_owned(),
                ..Default::default()
            },
            PenStyle::PS_DOT => Self {
                color: v.color_ref,
                dash_array: "2,2".to_owned(),
                ..Default::default()
            },
            PenStyle::PS_DASHDOT => Self {
                color: v.color_ref,
                dash_array: "6,2,2,2".to_owned(),
                ..Default::default()
            },
            PenStyle::PS_DASHDOTDOT => Self {
                color: v.color_ref,
                dash_array: "6,2,2,2,2,2".to_owned(),
                ..Default::default()
            },
            PenStyle::PS_NULL => Self { opacity: 0_f32, ..Default::default() },
            PenStyle::PS_ENDCAP_SQUARE => Self {
                color: v.color_ref,
                line_cap: "square".to_owned(),
                ..Default::default()
            },
            PenStyle::PS_JOIN_BEVEL => Self {
                color: v.color_ref,
                line_join: "bevel".to_owned(),
                ..Default::default()
            },
            PenStyle::PS_JOIN_MITER => Self {
                color: v.color_ref,
                line_join: "miter".to_owned(),
                ..Default::default()
            },
            // not supported or solid
            PenStyle::PS_INSIDEFRAME
            | PenStyle::PS_USERSTYLE
            | PenStyle::PS_ALTERNATE
            | PenStyle::PS_ENDCAP_FLAT
            | PenStyle::PS_SOLID => {
                Self { color: v.color_ref, ..Default::default() }
            }
        }
    }
}

impl Stroke {
    pub fn color(&self) -> String {
        css_color_from_color_ref(&self.color)
    }

    pub fn width(&self) -> i16 {
        self.width
    }

    pub fn opacity(&self) -> String {
        format!("{:.02}", self.opacity)
    }

    pub fn line_cap(&self) -> String {
        self.line_cap.clone()
    }

    pub fn dash_array(&self) -> String {
        self.dash_array.clone()
    }

    pub fn line_join(&self) -> String {
        self.line_join.clone()
    }
}
