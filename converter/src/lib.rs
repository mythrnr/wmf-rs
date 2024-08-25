mod converter;
mod generator;

pub use self::{converter::*, generator::*};

#[cfg(feature = "svg")]
mod svg;

#[cfg(feature = "svg")]
pub use self::svg::*;

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
pub enum GraphicsObject {
    Brush(wmf_core::Brush),
    Font(wmf_core::Font),
    Palette(wmf_core::Palette),
    Pen(wmf_core::Pen),
    Region(wmf_core::Region),
    Null,
}
