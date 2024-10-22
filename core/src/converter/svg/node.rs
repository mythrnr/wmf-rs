use crate::imports::*;

#[derive(Clone, Debug)]
pub struct Node {
    typ: NodeType,
    inner: Vec<Box<Node>>,
    attrs: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
enum NodeType {
    Node(String),
    Text(String),
}

impl Node {
    pub fn node(name: impl ToString) -> Self {
        Self {
            typ: NodeType::Node(name.to_string()),
            inner: vec![],
            attrs: BTreeMap::new(),
        }
    }

    pub fn text(value: impl ToString) -> Self {
        Self {
            typ: NodeType::Text(value.to_string()),
            inner: vec![],
            attrs: BTreeMap::new(),
        }
    }

    pub fn add(mut self, node: Node) -> Self {
        if matches!(self.typ, NodeType::Node(_)) {
            self.inner.push(Box::new(node));
        }

        self
    }

    pub fn set(mut self, name: impl ToString, value: impl ToString) -> Self {
        if matches!(self.typ, NodeType::Node(_)) {
            self.attrs.insert(name.to_string(), value.to_string());
        }

        self
    }

    fn escape_text(value: impl Into<String>) -> String {
        value
            .into()
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    fn escape_attr(value: impl Into<String>) -> String {
        Self::escape_text(value).replace('"', "&quot;").replace('\'', "&apos;")
    }
}

impl core::fmt::Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.typ {
            NodeType::Node(name) => {
                write!(
                    f,
                    "<{name} {}>{}</{name}>",
                    self.attrs
                        .iter()
                        .map(|(k, v)| {
                            format!(r#"{k}="{}""#, Self::escape_attr(v))
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                    self.inner
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                )
            }
            NodeType::Text(value) => {
                write!(f, "{}", Self::escape_text(value))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Data {
    commands: Vec<String>,
}

impl Data {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataMovetoCommands
    pub fn move_to(mut self, param: impl Into<Parameters>) -> Self {
        self.commands.push(format!("M {}", param.into().0));
        self
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    pub fn line_to(mut self, param: impl Into<Parameters>) -> Self {
        self.commands.push(format!("L {}", param.into().0));
        self
    }

    /// https://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    pub fn elliptical_arc_to(mut self, param: impl Into<Parameters>) -> Self {
        self.commands.push(format!("L {}", param.into().0));
        self
    }
}

impl core::fmt::Display for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.commands.join(" "))
    }
}

#[derive(Clone, Debug)]
pub struct Parameters(String);

impl From<String> for Parameters {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for Parameters {
    fn from(v: &str) -> Self {
        Self(v.to_owned())
    }
}

impl From<Vec<String>> for Parameters {
    fn from(v: Vec<String>) -> Self {
        Self(v.join(" "))
    }
}
