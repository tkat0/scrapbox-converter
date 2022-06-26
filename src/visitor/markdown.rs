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
    fn visit_bracket_emphasis(&mut self, emphasis: &Emphasis) -> Option<TransformCommand> {
        let h_level = (self.h1_level + 1).saturating_sub(emphasis.bold);
        if 0 < h_level
            && h_level <= self.h1_level
            && (self.bold_to_h || (!self.bold_to_h && emphasis.bold > 1))
        {
            Some(TransformCommand::Replace(Expr::new(ExprKind::Heading(
                Heading::new(&emphasis.text, h_level),
            ))))
        } else {
            None
        }
    }
}

pub struct MarkdownGenConfig {
    indent: String,
}

impl Default for MarkdownGenConfig {
    fn default() -> Self {
        Self {
            indent: "  ".to_string(),
        }
    }
}

pub struct MarkdownGen {
    document: String,
    config: MarkdownGenConfig,
}

impl MarkdownGen {
    pub fn new(config: MarkdownGenConfig) -> Self {
        Self {
            document: String::new(),
            config,
        }
    }

    pub fn generate(&mut self, page: &mut Page) -> String {
        self.visit(page);
        self.document.clone()
    }
}

impl Visitor for MarkdownGen {
    fn visit_page(&mut self, value: &mut Page) {
        for line in value.lines.iter_mut() {
            if let LineKind::List(list) = &line.kind {
                let indent = self.config.indent.repeat(list.level - 1);
                match &list.kind {
                    ListKind::Disc => self.document.push_str(&format!("{}* ", indent)),
                    ListKind::Decimal => self.document.push_str(&format!("{}1. ", indent)),
                    _ => {}
                }
            }
            self.visit_line(line);
            self.document.push_str("\n");
        }
    }

    fn visit_hashtag(&mut self, value: &HashTag) -> Option<TransformCommand> {
        self.document.push_str(&format!("#{}", value.value));
        None
    }

    fn visit_bracket_internal_link(&mut self, value: &InternalLink) -> Option<TransformCommand> {
        self.document.push_str(&format!("[[{}]]", value.title));
        None
    }

    fn visit_bracket_external_link(&mut self, value: &ExternalLink) -> Option<TransformCommand> {
        if let Some(title) = &value.title {
            self.document
                .push_str(&format!("[{}]({})", title, value.url));
        } else {
            self.document.push_str(&format!("{}", value.url));
        }
        None
    }

    fn visit_bracket_emphasis(&mut self, value: &Emphasis) -> Option<TransformCommand> {
        let mut tmp = value.text.clone();
        if value.bold > 0 {
            tmp = format!("**{}**", tmp);
        }
        if value.italic > 0 {
            tmp = format!("*{}*", tmp);
        }
        if value.strikethrough > 0 {
            tmp = format!("~~{}~~", tmp);
        }
        self.document.push_str(&tmp);
        None
    }

    fn visit_bracket_heading(&mut self, value: &Heading) -> Option<TransformCommand> {
        self.document.push_str(&format!(
            "{} {}",
            "#".repeat(value.level as usize),
            value.text
        ));
        None
    }

    fn visit_block_quate(&mut self, value: &BlockQuate) -> Option<TransformCommand> {
        self.document.push_str(&format!("`{}`", value.value));
        None
    }

    fn visit_code_block(&mut self, value: &CodeBlock) -> Option<TransformCommand> {
        self.document.push_str(&format!("```{}\n", value.file_name));
        for code in &value.value {
            self.document.push_str(&format!("{}\n", code));
        }
        self.document.push_str("```");
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
                vec![Expr::new(ExprKind::Emphasis(Emphasis::bold_level(
                    "text", 3,
                )))],
            )],
        };

        pass.visit(&mut page);

        assert_eq!(
            page.lines[0].values[0],
            Expr::new(ExprKind::Heading(Heading::new("text", 1)))
        )
    }

    #[test]
    fn pass_fallback_test() {
        let mut pass = MarkdownPass::default();

        let mut page = Page {
            lines: vec![Line::new(
                LineKind::Normal,
                vec![Expr::new(ExprKind::Emphasis(Emphasis::bold_level(
                    "text", 10,
                )))],
            )],
        };

        pass.visit(&mut page);

        assert_eq!(
            page.lines[0].values[0],
            Expr::new(ExprKind::Emphasis(Emphasis::bold_level("text", 10)))
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
                vec![Expr::new(ExprKind::Emphasis(Emphasis::bold_level(
                    "text", 1,
                )))],
            )],
        };

        pass.visit(&mut page);

        assert_eq!(
            page.lines[0].values[0],
            Expr::new(ExprKind::Heading(Heading::new("text", 3)))
        )
    }

    #[test]
    fn codegen_test() {
        let mut visitor = MarkdownGen::new(MarkdownGenConfig::default());

        let mut page = Page {
            lines: vec![
                Line::new(
                    LineKind::Normal,
                    vec![
                        Expr::new(ExprKind::Text(Text {
                            value: "abc ".to_string(),
                        })),
                        Expr::new(ExprKind::HashTag(HashTag {
                            value: "tag".to_string(),
                        })),
                        Expr::new(ExprKind::Text(Text {
                            value: " ".to_string(),
                        })),
                        Expr::new(ExprKind::ExternalLink(ExternalLink::new(
                            Some("Rust"),
                            "https://www.rust-lang.org/",
                        ))),
                    ],
                ),
                Line::new(
                    LineKind::List(List::new(ListKind::Disc, 2)),
                    vec![Expr::new(ExprKind::Text(Text {
                        value: "abc".to_string(),
                    }))],
                ),
                Line::new(
                    LineKind::Normal,
                    vec![Expr::new(ExprKind::CodeBlock(CodeBlock::new(
                        "hello.rs",
                        vec!["fn main() {", "    println(\"Hello, World!\");", "}"],
                    )))],
                ),
            ],
        };

        let markdown = visitor.generate(&mut page);

        assert_eq!(
            markdown,
            "abc #tag [Rust](https://www.rust-lang.org/)\n  * abc\n```hello.rs\nfn main() {\n    println(\"Hello, World!\");\n}\n```\n"
        )
    }
}
