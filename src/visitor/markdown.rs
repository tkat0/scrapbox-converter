use super::{TransformCommand, Visitor};
use crate::ast::*;

pub struct MarkdownPass {
    // Examples:
    // - `h1_level1` == 3: [*** text] -> `# text`
    // - `h1_level1` == 3: [** text] -> `## text`
    // - `h1_level1` == 3: [* text] -> `### text` or `**text**` (see `bold_to_h`)
    // - `h1_level1` == 5: [***** text] -> `# text`
    pub h1_level: u8,
    // If true, `[* bold]` -> `**bold**`.
    // If false, `[* bold]` -> `### bold`.
    pub bold_to_h: bool,
}

impl Default for MarkdownPass {
    fn default() -> Self {
        Self {
            h1_level: 3,
            bold_to_h: false,
        }
    }
}

impl Visitor for MarkdownPass {
    fn visit_bracket_decoration(&mut self, decoration: &Decoration) -> Option<TransformCommand> {
        let h_level = (self.h1_level + 1).saturating_sub(decoration.bold);
        if 0 < h_level
            && h_level <= self.h1_level
            && (self.bold_to_h || (!self.bold_to_h && decoration.bold > 1))
        {
            Some(TransformCommand::Replace(Syntax::new(SyntaxKind::Bracket(
                Bracket::new(BracketKind::Heading(Heading::new(
                    &decoration.text,
                    h_level,
                ))),
            ))))
        } else {
            None
        }
    }
}

pub struct MarkdownGen {
    document: String,
}

impl MarkdownGen {
    pub fn new() -> Self {
        Self {
            document: String::new(),
        }
    }

    pub fn generate(&mut self, page: &mut Page) -> String {
        self.visit(page);
        self.document.clone()
    }
}

impl Visitor for MarkdownGen {
    fn visit_page(&mut self, page: &mut Page) {
        for line in page.lines.iter_mut() {
            self.visit_line(line);
            self.document.push_str("\n");
        }
    }

    fn visit_hashtag(&mut self, hashtag: &HashTag) -> Option<TransformCommand> {
        self.document.push_str(&format!("#{}", hashtag.value));
        None
    }

    fn visit_bracket_internal_link(&mut self, link: &InternalLink) -> Option<TransformCommand> {
        self.document
            .push_str(&format!("[{t}]({t}.md)", t = link.title));
        None
    }

    fn visit_bracket_external_link(&mut self, link: &ExternalLink) -> Option<TransformCommand> {
        if let Some(title) = &link.title {
            self.document
                .push_str(&format!("[{}]({})", title, link.url));
        } else {
            self.document.push_str(&format!("{}", link.url));
        }
        None
    }

    fn visit_bracket_decoration(&mut self, decoration: &Decoration) -> Option<TransformCommand> {
        let mut tmp = decoration.text.clone();
        if decoration.bold > 0 {
            tmp = format!("**{}**", tmp);
        }
        if decoration.italic > 0 {
            tmp = format!("*{}*", tmp);
        }
        if decoration.strikethrough > 0 {
            tmp = format!("~~{}~~", tmp);
        }
        self.document.push_str(&tmp);
        None
    }

    fn visit_bracket_heading(&mut self, heading: &Heading) -> Option<TransformCommand> {
        self.document.push_str(&format!(
            "{} {}",
            "#".repeat(heading.level as usize),
            heading.text
        ));
        None
    }

    fn visit_text(&mut self, text: &Text) -> Option<TransformCommand> {
        self.document.push_str(&format!("{}", text.value));
        None
    }
}

mod test {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn pass_test() {
        let mut pass = MarkdownPass::default();

        let mut page = Page {
            lines: vec![Line::new(
                LineKind::Normal,
                vec![Syntax::new(SyntaxKind::Bracket(Bracket::new(
                    BracketKind::Decoration(Decoration::bold_level("text", 3)),
                )))],
            )],
        };

        pass.visit(&mut page);

        assert_eq!(
            page.lines[0].values[0],
            Syntax::new(SyntaxKind::Bracket(Bracket::new(BracketKind::Heading(
                Heading::new("text", 1)
            ))))
        )
    }

    #[test]
    fn pass_fallback_test() {
        let mut pass = MarkdownPass::default();

        let mut page = Page {
            lines: vec![Line::new(
                LineKind::Normal,
                vec![Syntax::new(SyntaxKind::Bracket(Bracket::new(
                    BracketKind::Decoration(Decoration::bold_level("text", 10)),
                )))],
            )],
        };

        pass.visit(&mut page);

        assert_eq!(
            page.lines[0].values[0],
            Syntax::new(SyntaxKind::Bracket(Bracket::new(BracketKind::Decoration(
                Decoration::bold_level("text", 10)
            ))))
        )
    }

    #[test]
    fn pass_bold_to_h_test() {
        let mut pass = MarkdownPass {
            h1_level: 3,
            bold_to_h: true,
        };

        // TODO(tkat0): not supoprted: `[*-/ mix]` -> `### *~~mix~~*` (but `### mix`)
        let mut page = Page {
            lines: vec![Line::new(
                LineKind::Normal,
                vec![Syntax::new(SyntaxKind::Bracket(Bracket::new(
                    BracketKind::Decoration(Decoration::bold_level("text", 1)),
                )))],
            )],
        };

        pass.visit(&mut page);

        assert_eq!(
            page.lines[0].values[0],
            Syntax::new(SyntaxKind::Bracket(Bracket::new(BracketKind::Heading(
                Heading::new("text", 3)
            ))))
        )
    }

    #[test]
    fn codegen_test() {
        let mut visitor = MarkdownGen::new();

        let mut page = Page {
            lines: vec![Line::new(
                LineKind::Normal,
                vec![
                    Syntax {
                        kind: SyntaxKind::Text(Text {
                            value: "abc ".to_string(),
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

        let markdown = visitor.generate(&mut page);

        assert_eq!(markdown, "abc #tag [Rust](https://www.rust-lang.org/)\n")
    }
}
