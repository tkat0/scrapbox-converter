#[derive(Debug, Clone, PartialEq, Default)]
pub struct Page {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct NodeId(usize);

/// When parsing the AST, NodeId is given this dummy Id.
/// Then, during a later phase, it will be replaced.
pub const DUMMY_NODE_ID: NodeId = NodeId(usize::MIN);

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Paragraph(Paragraph),
    List(List),
    HashTag(HashTag),
    InternalLink(InternalLink),
    ExternalLink(ExternalLink),
    Emphasis(Emphasis),
    Heading(Heading),
    BlockQuate(BlockQuate),
    CodeBlock(CodeBlock),
    Table(Table),
    Image(Image),
    Math(Math),
    Text(Text),
    Nop,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            id: DUMMY_NODE_ID,
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub children: Vec<Node>,
}

impl Paragraph {
    pub fn new(children: Vec<Node>) -> Self {
        Self { children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub children: Vec<ListItem>,
}

impl List {
    pub fn new(children: Vec<ListItem>) -> Self {
        Self { children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListKind {
    Disc,
    Decimal,
    Alphabet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub kind: ListKind,
    pub level: usize,
    pub children: Vec<Node>,
}

impl ListItem {
    pub fn new(kind: ListKind, level: usize, children: Vec<Node>) -> Self {
        Self {
            kind,
            level,
            children,
        }
    }

    pub fn disc(level: usize, children: Vec<Node>) -> Self {
        Self {
            kind: ListKind::Disc,
            level,
            children,
        }
    }

    pub fn decimal(level: usize, children: Vec<Node>) -> Self {
        Self {
            kind: ListKind::Decimal,
            level,
            children,
        }
    }

    pub fn alphabet(level: usize, children: Vec<Node>) -> Self {
        Self {
            kind: ListKind::Alphabet,
            level,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HashTag {
    pub value: String,
}

impl HashTag {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockQuate {
    pub value: String,
}

impl BlockQuate {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub file_name: String,
    pub children: Vec<String>,
}

impl CodeBlock {
    pub fn new(file_name: &str, children: Vec<&str>) -> Self {
        Self {
            file_name: file_name.to_string(),
            children: children.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub name: String,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(name: &str, header: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            name: name.into(),
            header,
            rows,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Text {
    pub value: String,
}

impl Text {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InternalLink {
    pub title: String,
}

impl InternalLink {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExternalLink {
    pub title: Option<String>,
    pub url: String,
}

impl ExternalLink {
    pub fn new(title: Option<&str>, url: &str) -> Self {
        Self {
            title: title.map(String::from),
            url: url.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Heading {
    pub text: String,
    pub level: usize,
}

// TODO(tkat0): replace &str with Node to support "# `code`"
impl Heading {
    pub fn new(text: &str, level: usize) -> Self {
        Self {
            text: text.to_string(),
            level,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Emphasis {
    pub text: String,
    pub bold: usize,
    pub italic: usize,
    pub strikethrough: usize,
}

impl Emphasis {
    pub fn new(text: &str, bold: usize, italic: usize, strikethrough: usize) -> Self {
        Self {
            text: text.to_string(),
            bold,
            italic,
            strikethrough,
        }
    }

    pub fn bold(text: &str) -> Self {
        Self {
            text: text.to_string(),
            bold: 1,
            ..Default::default()
        }
    }

    pub fn bold_level(text: &str, level: usize) -> Self {
        Self {
            text: text.to_string(),
            bold: level,
            ..Default::default()
        }
    }

    pub fn italic(text: &str) -> Self {
        Self {
            text: text.to_string(),
            italic: 1,
            ..Default::default()
        }
    }

    pub fn strikethrough(text: &str) -> Self {
        Self {
            text: text.to_string(),
            strikethrough: 1,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Image {
    // TODO(tkat0): title
    pub uri: String,
}

impl Image {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Math {
    pub value: String,
}

impl Math {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HtmlTag {
    /// "<tag>", "<tag />", "</tag>"
    pub value: String,
}

impl HtmlTag {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ast_test() {
        let page = Page {
            nodes: vec![
                Node::new(NodeKind::List(List::new(vec![ListItem::new(
                    ListKind::Disc,
                    1,
                    vec![
                        Node::new(NodeKind::Text(Text::new("abc"))),
                        Node::new(NodeKind::HashTag(HashTag::new("tag"))),
                        Node::new(NodeKind::Text(Text::new(" "))),
                        Node::new(NodeKind::ExternalLink(ExternalLink::new(
                            Some("Rust"),
                            "https://www.rust-lang.org/",
                        ))),
                    ],
                )]))),
                Node::new(NodeKind::BlockQuate(BlockQuate::new("git"))),
            ],
        };

        dbg!(page);
    }
}
