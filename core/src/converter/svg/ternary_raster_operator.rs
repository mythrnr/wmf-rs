use ::svg::{
    node::element::{Image, Rectangle},
    Node,
};

use crate::{
    converter::{
        svg::{util::url_string, Fill},
        *,
    },
    parser::*,
};

#[derive(Clone, Debug, thiserror::Error)]
pub enum TernaryRasterOperationError {
    #[error("no brush specified: {cause}")]
    NoBrush { cause: String },
    #[error("no source bitmap specified: {cause}")]
    NoSource { cause: String },
}

pub struct TernaryRasterOperator {
    operation: TernaryRasterOperation,
    x: i16,
    y: i16,
    height: i16,
    width: i16,
    brush: Option<Brush>,
    source: Option<Source>,
}

enum Source {
    Bitmap16(Bitmap16),
    Bitmap(DeviceIndependentBitmap),
}

impl TernaryRasterOperator {
    pub fn new(
        operation: TernaryRasterOperation,
        x: i16,
        y: i16,
        height: i16,
        width: i16,
    ) -> Self {
        Self { operation, x, y, height, width, brush: None, source: None }
    }

    pub fn brush(self, brush: Brush) -> Self {
        Self { brush: brush.into(), ..self }
    }

    pub fn source_bitmap16(self, source: Bitmap16) -> Self {
        Self { source: Source::Bitmap16(source).into(), ..self }
    }

    pub fn source_bitmap(self, source: DeviceIndependentBitmap) -> Self {
        Self { source: Source::Bitmap(source).into(), ..self }
    }

    pub fn run(
        self,
        definitions: &mut Vec<Box<dyn Node>>,
    ) -> Result<Option<Box<dyn ::svg::Node>>, TernaryRasterOperationError> {
        if self.operation.use_selected_brush() && self.brush.is_none() {
            return Err(TernaryRasterOperationError::NoBrush {
                cause: format!(
                    "TernaryRasterOperation {:?} cannot access brush.",
                    self.operation,
                ),
            });
        };

        if self.operation.use_source() && self.source.is_none() {
            return Err(TernaryRasterOperationError::NoSource {
                cause: format!(
                    "TernaryRasterOperation {:?} cannot access source bitmap.",
                    self.operation,
                ),
            });
        };

        let result: Box<dyn Node> = match self.operation {
            TernaryRasterOperation::BLACKNESS => {
                let rect = Rectangle::new()
                    .set("x", self.x)
                    .set("y", self.y)
                    .set("width", self.width)
                    .set("height", self.height)
                    .set("stroke", "none")
                    .set("fill", "black");

                Box::new(rect)
            }
            TernaryRasterOperation::SRCCOPY => {
                let bitmap = match self.source.unwrap() {
                    Source::Bitmap16(data) => Bitmap::from(data),
                    Source::Bitmap(data) => Bitmap::from(data),
                };

                Box::new(
                    Image::new()
                        .set("x", self.x)
                        .set("y", self.y)
                        .set("width", self.width)
                        .set("height", self.height)
                        .set("href", bitmap.as_data_url()),
                )
            }
            TernaryRasterOperation::PATCOPY => {
                let fill = match Fill::from(self.brush.clone().unwrap()) {
                    Fill::Pattern { pattern } => {
                        let id = self.issue_id(&definitions);
                        definitions.push(pattern.set("id", id.as_str()).into());
                        url_string(format!("#{id}"))
                    }
                    Fill::Value { value } => value,
                };

                let rect = Rectangle::new()
                    .set("x", self.x)
                    .set("y", self.y)
                    .set("width", self.width)
                    .set("height", self.height)
                    .set("fill", fill.as_str());

                Box::new(rect)
            }
            TernaryRasterOperation::WHITENESS => {
                let rect = Rectangle::new()
                    .set("x", self.x)
                    .set("y", self.y)
                    .set("width", self.width)
                    .set("height", self.height)
                    .set("stroke", "none")
                    .set("fill", "white");

                Box::new(rect)
            }
            operation => {
                tracing::info!(
                    ?operation,
                    "TernaryRasterOperation is not implemented"
                );

                return Ok(None);
            }
        };

        Ok(Some(result))
    }

    #[inline]
    fn issue_id(&self, definitions: &Vec<Box<dyn Node>>) -> String {
        format!("rop_pat{}", definitions.len())
    }
}

impl From<ColorRef> for RGBQuad {
    fn from(v: ColorRef) -> Self {
        let ColorRef { red, green, blue, reserved } = v;
        Self { red, green, blue, reserved }
    }
}
