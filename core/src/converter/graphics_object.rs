use crate::parser::*;

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
pub struct GraphicsObjects(Vec<GraphicsObject>);

impl GraphicsObjects {
    pub fn new(v: usize) -> Self {
        Self(vec![GraphicsObject::Null; v])
    }

    pub fn delete(&mut self, i: usize) {
        self.0[i] = GraphicsObject::Null;
    }

    pub fn get(&self, i: usize) -> &GraphicsObject {
        self.0.get(i).expect("should be set")
    }

    pub fn push(&mut self, g: GraphicsObject) {
        for (i, v) in self.0.iter_mut().enumerate() {
            if matches!(&v, GraphicsObject::Null) {
                self.0[i] = g;
                break;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectedGraphicsObject {
    pub brush: Brush,
    pub font: Font,
    pub _palette: Option<Palette>,
    pub pen: Pen,
    pub _region: Option<Region>,
}

impl Default for SelectedGraphicsObject {
    fn default() -> Self {
        use std::collections::BTreeSet;

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
            },
            _palette: None,
            pen: Pen {
                style: BTreeSet::from_iter([PenStyle::PS_SOLID]),
                width: PointS { x: 1, y: 0 },
                color_ref: ColorRef::black(),
            },
            _region: None,
        }
    }
}

impl SelectedGraphicsObject {
    pub fn brush(self, brush: Brush) -> Self {
        Self { brush, ..self }
    }

    pub fn font(self, font: Font) -> Self {
        Self { font, ..self }
    }

    pub fn palette(self, palette: Palette) -> Self {
        Self { _palette: palette.into(), ..self }
    }

    pub fn pen(self, pen: Pen) -> Self {
        Self { pen, ..self }
    }

    pub fn region(self, region: Region) -> Self {
        Self { _region: region.into(), ..self }
    }
}