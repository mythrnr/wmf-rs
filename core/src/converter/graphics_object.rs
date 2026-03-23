use crate::{imports::*, parser::*};

#[derive(Clone, Debug)]
pub enum GraphicsObject {
    Brush(Brush),
    Font(Font),
    Palette(Palette),
    Pen(Pen),
    Region(Region),
    Null,
}

#[derive(Clone, Debug)]
pub struct GraphicsObjects(Vec<GraphicsObject>, GraphicsObject);

impl GraphicsObjects {
    pub fn new(v: usize) -> Self {
        Self(vec![GraphicsObject::Null; v], GraphicsObject::Null)
    }

    pub fn delete(&mut self, i: usize) {
        if let Some(slot) = self.0.get_mut(i) {
            *slot = GraphicsObject::Null;
        } else {
            warn!(
                index = i,
                capacity = self.0.len(),
                "object table index out of bounds on delete",
            );
        }
    }

    pub fn get(&self, i: usize) -> &GraphicsObject {
        self.0.get(i).unwrap_or(&self.1)
    }

    pub fn push(&mut self, g: GraphicsObject) {
        for (i, v) in self.0.iter_mut().enumerate() {
            if matches!(&v, GraphicsObject::Null) {
                self.0[i] = g;
                return;
            }
        }

        warn!(capacity = self.0.len(), "object table is full, ignoring push");
    }
}

#[derive(Clone, Debug)]
pub struct SelectedGraphicsObject {
    pub brush: Brush,
    pub font: Font,
    pub palette: Option<Palette>,
    pub pen: Pen,
    pub region: Option<Region>,
}

impl Default for SelectedGraphicsObject {
    fn default() -> Self {
        SelectedGraphicsObject {
            brush: Brush::Null,
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
                fallback_facename: vec!["System".to_owned()],
            },
            palette: None,
            pen: Pen {
                style: PenStyleSubsection {
                    style: PenStyle::PS_SOLID,
                    end_cap: PenStyle::PS_ENDCAP_FLAT,
                    line_join: PenStyle::PS_JOIN_MITER,
                    typ: PenStyle::PS_SOLID,
                },
                width: PointS { x: 1, y: 0 },
                color_ref: ColorRef::black(),
            },
            region: None,
        }
    }
}

impl SelectedGraphicsObject {
    pub fn set_brush(&mut self, brush: Brush) {
        self.brush = brush;
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn set_palette(&mut self, palette: Palette) {
        self.palette = palette.into();
    }

    pub fn set_pen(&mut self, pen: Pen) {
        self.pen = pen;
    }

    pub fn set_region(&mut self, region: Region) {
        self.region = region.into();
    }
}
