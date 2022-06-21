#[derive(Debug, PartialEq)]
pub struct Page {
    pub lines: Vec<Line>,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub items: Vec<Syntax>,
}

#[derive(Debug, PartialEq)]
pub struct Syntax {
    pub kind: SyntaxKind,
}

#[derive(Debug, PartialEq)]
pub enum SyntaxKind {
    HashTag(HashTag),
    Bracket(Bracket),
    Text(Text),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Bracket {
    pub kind: BracketKind,
}

#[derive(Debug, PartialEq)]
pub enum BracketKind {
    InternalLink(InternalLink),
    ExternalLink(ExternalLink),
    Decoration,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

mod test {
    use super::*;

    #[test]
    fn aaa() {
        let page = Page {
            lines: vec![Line {
                items: vec![
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
                        kind: SyntaxKind::Bracket(Bracket {
                            kind: BracketKind::ExternalLink(ExternalLink {
                                title: None,
                                url: "https://www.rust-lang.org/".to_string(),
                            }),
                        }),
                    },
                ],
            }],
        };

        dbg!(page);
    }
}
