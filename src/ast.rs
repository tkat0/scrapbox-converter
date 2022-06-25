#[derive(Debug, Clone, PartialEq, Default)]
pub struct Page {
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub kind: LineKind,
    pub values: Vec<Syntax>,
}

impl Line {
    pub fn new(kind: LineKind, values: Vec<Syntax>) -> Self {
        Self { kind, values }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineKind {
    Normal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Syntax {
    pub kind: SyntaxKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxKind {
    HashTag(HashTag),
    Bracket(Bracket),
    Text(Text),
}

impl Syntax {
    pub fn new(kind: SyntaxKind) -> Self {
        Self { kind }
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
pub struct Bracket {
    pub kind: BracketKind,
}

impl Bracket {
    pub fn new(kind: BracketKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BracketKind {
    InternalLink(InternalLink),
    ExternalLink(ExternalLink),
    Decoration(Decoration),
    Heading(Heading),
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
    pub level: u8,
}

impl Heading {
    pub fn new(text: &str, level: u8) -> Self {
        Self {
            text: text.to_string(),
            level,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Decoration {
    pub text: String,
    pub bold: u8,
    pub italic: u8,
    pub strikethrough: u8,
}

impl Decoration {
    pub fn new(text: &str, bold: u8, italic: u8, strikethrough: u8) -> Self {
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

    pub fn bold_level(text: &str, level: u8) -> Self {
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

mod test {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn ast_test() {
        let page = Page {
            lines: vec![Line::new(
                LineKind::Normal,
                vec![
                    Syntax {
                        kind: SyntaxKind::Text(Text {
                            value: "abc".to_string(),
                        }),
                    },
                    Syntax {
                        kind: SyntaxKind::HashTag(HashTag {
                            value: "tag".to_string(),
                        }),
                    },
                    Syntax {
                        kind: SyntaxKind::Text(Text {
                            value: " ".to_string(),
                        }),
                    },
                    Syntax {
                        kind: SyntaxKind::Bracket(Bracket::new(BracketKind::ExternalLink(
                            ExternalLink::new(Some("Rust"), "https://www.rust-lang.org/"),
                        ))),
                    },
                ],
            )],
        };

        dbg!(page);
    }
}
