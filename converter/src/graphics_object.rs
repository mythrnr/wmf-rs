use wmf_core::*;

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

#[derive(Clone, Debug, Default)]
pub struct SelectedGraphicsObject {
    pub brush: Option<Brush>,
    pub font: Option<Font>,
    pub palette: Option<Palette>,
    pub pen: Option<Pen>,
    pub region: Option<Region>,
}

impl SelectedGraphicsObject {
    pub fn brush(self, brush: Brush) -> Self {
        Self { brush: brush.into(), ..self }
    }

    pub fn font(self, font: Font) -> Self {
        Self { font: font.into(), ..self }
    }

    pub fn palette(self, palette: Palette) -> Self {
        Self { palette: palette.into(), ..self }
    }

    pub fn pen(self, pen: Pen) -> Self {
        Self { pen: pen.into(), ..self }
    }

    pub fn region(self, region: Region) -> Self {
        Self { region: region.into(), ..self }
    }
}
