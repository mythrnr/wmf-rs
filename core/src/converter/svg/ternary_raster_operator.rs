use crate::converter::{
    svg::{Fill, node::Node, util::url_string},
    *,
};

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum TernaryRasterOperationError {
    #[snafu(display("no brush specified: {cause}"))]
    NoBrush { cause: String },
    #[snafu(display("no source bitmap specified: {cause}"))]
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

impl Source {
    /// Convert the source bitmap into a converter::Bitmap
    fn into_bitmap(self) -> Bitmap {
        match self {
            Source::Bitmap16(data) => {
                let bitmap = crate::parser::DeviceIndependentBitmap::from(data);
                crate::converter::Bitmap::from(bitmap)
            }
            Source::Bitmap(data) => Bitmap::from(data),
        }
    }
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

    pub fn brush(mut self, brush: Brush) -> Self {
        self.brush = brush.into();
        self
    }

    pub fn source_bitmap16(mut self, source: Bitmap16) -> Self {
        self.source = Source::Bitmap16(source).into();
        self
    }

    pub fn source_bitmap(mut self, source: DeviceIndependentBitmap) -> Self {
        self.source = Source::Bitmap(source).into();
        self
    }

    pub fn run(
        self,
        definitions: &mut Vec<Node>,
    ) -> Result<Option<Node>, TernaryRasterOperationError> {
        if self.operation.use_selected_brush() && self.brush.is_none() {
            return Err(TernaryRasterOperationError::NoBrush {
                cause: format!(
                    "TernaryRasterOperation {:?} cannot access brush.",
                    self.operation,
                ),
            });
        }

        if self.operation.use_source() && self.source.is_none() {
            return Err(TernaryRasterOperationError::NoSource {
                cause: format!(
                    "TernaryRasterOperation {:?} cannot access source bitmap.",
                    self.operation,
                ),
            });
        }

        let result: Node = match self.operation {
            TernaryRasterOperation::BLACKNESS => self.black_rect(),
            TernaryRasterOperation::WHITENESS => self.white_rect(),
            TernaryRasterOperation::SRCCOPY => self.source_image(),
            TernaryRasterOperation::PATCOPY => self.pattern_rect(definitions),
            TernaryRasterOperation::NOTSRCCOPY => {
                self.source_image_inverted(definitions)
            }
            TernaryRasterOperation::SRCPAINT => {
                self.source_image_blended("screen")
            }
            TernaryRasterOperation::SRCAND => {
                self.source_image_blended("multiply")
            }
            TernaryRasterOperation::SRCINVERT => {
                self.source_image_blended("difference")
            }
            TernaryRasterOperation::SRCERASE => {
                // S & ~D: no exact SVG equivalent; render
                // source as approximation
                info!(
                    "SRCERASE (S & ~D): approximated as source copy \
                     (destination masking not available)",
                );
                self.source_image()
            }
            TernaryRasterOperation::NOTSRCERASE => {
                // ~(S | D): no exact SVG equivalent; render
                // inverted source as approximation
                info!(
                    "NOTSRCERASE ~(S | D): approximated as inverted source \
                     (destination not available)",
                );
                self.source_image_inverted(definitions)
            }
            TernaryRasterOperation::DSTINVERT => {
                // ~D: white rect + difference blend to
                // invert the background
                self.white_rect().set("style", "mix-blend-mode: difference;")
            }
            TernaryRasterOperation::PATINVERT => {
                // P ^ D: pattern + difference blend
                self.pattern_rect(definitions)
                    .set("style", "mix-blend-mode: difference;")
            }
            TernaryRasterOperation::MERGECOPY => {
                // P & S: overlay pattern on source with
                // multiply blend (P * S approximates P & S)
                self.source_with_pattern(definitions, "multiply")
            }
            TernaryRasterOperation::MERGEPAINT => {
                // ~S | D: inverted source + screen blend
                let img = self.source_image_inverted(definitions);
                img.set("style", "mix-blend-mode: screen;")
            }
            TernaryRasterOperation::PATPAINT => {
                // P | ~S | D: combine inverted source and
                // pattern, both screened onto destination
                self.inverted_source_with_pattern(definitions, "screen")
            }
            TernaryRasterOperation::PN => {
                // ~P: inverted pattern
                let filter_id = Self::create_invert_filter(definitions);
                self.pattern_rect(definitions)
                    .set("filter", url_string(format!("#{filter_id}").as_str()))
            }
            operation => {
                info!(?operation, "TernaryRasterOperation is not implemented",);

                return Ok(None);
            }
        };

        Ok(Some(result))
    }

    /// Generate a black-filled rectangle
    fn black_rect(&self) -> Node {
        Node::new("rect")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("stroke", "none")
            .set("fill", "black")
    }

    /// Generate a white-filled rectangle
    fn white_rect(&self) -> Node {
        Node::new("rect")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("stroke", "none")
            .set("fill", "white")
    }

    /// Render the source bitmap as an image
    fn source_image(self) -> Node {
        let bitmap = self.source.unwrap().into_bitmap();

        Node::new("image")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("href", bitmap.as_data_url())
    }

    /// Render the source bitmap with color inversion
    fn source_image_inverted(self, definitions: &mut Vec<Node>) -> Node {
        let filter_id = Self::create_invert_filter(definitions);
        let bitmap = self.source.unwrap().into_bitmap();

        Node::new("image")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("href", bitmap.as_data_url())
            .set("filter", url_string(format!("#{filter_id}").as_str()))
    }

    /// Render the source bitmap with a blend mode
    fn source_image_blended(self, blend_mode: &str) -> Node {
        let bitmap = self.source.unwrap().into_bitmap();

        Node::new("image")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("href", bitmap.as_data_url())
            .set("style", format!("mix-blend-mode: {blend_mode};"))
    }

    /// Render source image composited with pattern using the
    /// given blend mode. Used for operations like MERGECOPY
    /// (P & S) where both source and pattern interact.
    fn source_with_pattern(
        self,
        definitions: &mut Vec<Node>,
        blend_mode: &str,
    ) -> Node {
        let pattern = self.pattern_rect(definitions);
        let bitmap = self.source.unwrap().into_bitmap();
        let source = Node::new("image")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("href", bitmap.as_data_url())
            .set("style", format!("mix-blend-mode: {blend_mode};"));

        // Isolate the group so the blend mode only applies
        // between the source and the pattern, not the
        // destination underneath.
        Node::new("g")
            .set("style", "isolation: isolate;")
            .add(pattern)
            .add(source)
    }

    /// Render inverted source composited with pattern, both
    /// blended onto the destination. Used for operations like
    /// PATPAINT (P | ~S | D).
    fn inverted_source_with_pattern(
        self,
        definitions: &mut Vec<Node>,
        blend_mode: &str,
    ) -> Node {
        let filter_id = Self::create_invert_filter(definitions);
        let pattern = self
            .pattern_rect(definitions)
            .set("style", format!("mix-blend-mode: {blend_mode};"));
        let bitmap = self.source.unwrap().into_bitmap();
        let source = Node::new("image")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("href", bitmap.as_data_url())
            .set("filter", url_string(format!("#{filter_id}").as_str()))
            .set("style", format!("mix-blend-mode: {blend_mode};"));

        Node::new("g").add(source).add(pattern)
    }

    /// Generate a rectangle filled with the pattern brush
    fn pattern_rect(&self, definitions: &mut Vec<Node>) -> Node {
        let fill = match Fill::from(self.brush.as_ref().unwrap()) {
            Fill::Pattern { pattern } => {
                let id = Self::issue_id(definitions);
                definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };

        Node::new("rect")
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("fill", fill.as_str())
    }

    /// Add a color-inversion SVG filter to definitions and return its ID
    fn create_invert_filter(definitions: &mut Vec<Node>) -> String {
        let id = format!("rop_inv{}", definitions.len());
        let filter = Node::new("filter").set("id", id.as_str()).add(
            Node::new("feComponentTransfer")
                .add(
                    Node::new("feFuncR")
                        .set("type", "table")
                        .set("tableValues", "1 0"),
                )
                .add(
                    Node::new("feFuncG")
                        .set("type", "table")
                        .set("tableValues", "1 0"),
                )
                .add(
                    Node::new("feFuncB")
                        .set("type", "table")
                        .set("tableValues", "1 0"),
                ),
        );

        definitions.push(filter);
        id
    }

    #[inline]
    fn issue_id(definitions: &[Node]) -> String {
        format!("rop_pat{}", definitions.len())
    }
}

impl From<ColorRef> for RGBQuad {
    fn from(v: ColorRef) -> Self {
        let ColorRef { red, green, blue, reserved } = v;
        Self { red, green, blue, reserved }
    }
}
