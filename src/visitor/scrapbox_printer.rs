use super::{walk_paragraph, TransformCommand, Visitor};
use crate::ast::*;

pub struct ScrapboxPrinterConfig {
    indent: String,
    h1_mapping: usize,
}

impl Default for ScrapboxPrinterConfig {
    fn default() -> Self {
        Self {
            indent: "\t".into(),
            h1_mapping: 4,
        }
    }
}

pub struct ScrapboxPrinter {
    document: String,
    config: ScrapboxPrinterConfig,
}

impl ScrapboxPrinter {
    pub fn new(config: ScrapboxPrinterConfig) -> Self {
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

impl Visitor for ScrapboxPrinter {
    fn visit_paragraph(&mut self, value: &mut Paragraph) -> Option<TransformCommand> {
        walk_paragraph(self, value);
        self.document.push_str("\n");
        None
    }

    fn visit_list(&mut self, value: &mut List) -> Option<TransformCommand> {
        let mut number = 1;
        for item in value.children.iter_mut() {
            let indent = self.config.indent.repeat(item.level + 1); // TODO(tkat0): consistency
            match &item.kind {
                ListKind::Disc => self.document.push_str(&format!("{}", indent)),
                ListKind::Decimal => self.document.push_str(&format!("{}{}. ", indent, number)),
                _ => {}
            }

            if item.kind == ListKind::Decimal {
                number += 1;
            } else {
                number = 1; // reset
            }

            for node in item.children.iter_mut() {
                self.visit_node(node);
            }
            self.document.push_str("\n");
        }
        None
    }

    fn visit_hashtag(&mut self, value: &HashTag) -> Option<TransformCommand> {
        self.document.push_str(&format!("#{}", value.value));
        None
    }

    fn visit_internal_link(&mut self, value: &InternalLink) -> Option<TransformCommand> {
        self.document.push_str(&format!("[{}]", value.title));
        None
    }

    fn visit_external_link(&mut self, value: &ExternalLink) -> Option<TransformCommand> {
        if let Some(title) = &value.title {
            self.document
                .push_str(&format!("[{} {}]", title, value.url));
        } else {
            self.document.push_str(&format!("[{}]", value.url));
        }
        None
    }

    fn visit_emphasis(&mut self, value: &Emphasis) -> Option<TransformCommand> {
        self.document.push_str("[");
        if value.bold > 0 {
            self.document.push_str("*");
        }
        if value.italic > 0 {
            self.document.push_str("/");
        }
        if value.strikethrough > 0 {
            self.document.push_str("-");
        }
        self.document.push_str(&format!(" {}]", value.text));
        None
    }

    fn visit_heading(&mut self, value: &Heading) -> Option<TransformCommand> {
        let level = if self.config.h1_mapping + 1 > value.level {
            self.config.h1_mapping - value.level + 1
        } else {
            1
        };
        self.document
            .push_str(&format!("[{} {}]\n", "*".repeat(level), value.text));
        None
    }

    fn visit_block_quate(&mut self, value: &BlockQuate) -> Option<TransformCommand> {
        self.document.push_str(&format!("`{}`", value.value));
        None
    }

    fn visit_code_block(&mut self, value: &CodeBlock) -> Option<TransformCommand> {
        self.document
            .push_str(&format!("code:{}\n", value.file_name));
        for code in &value.children {
            self.document.push_str(&format!(" {}\n", code));
        }
        None
    }

    fn visit_table(&mut self, value: &Table) -> Option<TransformCommand> {
        if value.header.is_empty() {
            return None;
        }

        self.document.push_str(&format!("table:{}\n", value.name));
        self.document
            .push_str(&format!(" {}\n", value.header.join("\t")));
        for row in &value.rows {
            if row.is_empty() {
                break;
            }
            self.document.push_str(&format!(" {}\n", row.join("\t")));
        }
        None
    }

    fn visit_image(&mut self, value: &Image) -> Option<TransformCommand> {
        self.document.push_str(&format!("[{}]", value.uri));
        None
    }

    fn visit_math(&mut self, value: &Math) -> Option<TransformCommand> {
        self.document.push_str(&format!("[${}]", value.value));
        None
    }

    fn visit_text(&mut self, value: &Text) -> Option<TransformCommand> {
        self.document.push_str(&format!("{}", value.value));
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    // #[test]
    // fn pass_test() {
    //     let mut pass = ScrapboxPass::default();

    //     assert_eq!(
    //         pass.visit_emphasis(&mut Emphasis::bold_level("text", 3)),
    //         Some(TransformCommand::Replace(NodeKind::Heading(Heading::new(
    //             "text", 1
    //         ))))
    //     );
    // }

    // #[test]
    // fn pass_bold_to_h_test() {
    //     let mut pass = ScrapboxPass {
    //         h1_level: 3,
    //         bold_to_h: true,
    //     };

    //     // TODO(tkat0): not supoprted: `[*-/ mix]` -> `### *~~mix~~*` (but `### mix`)
    //     assert_eq!(
    //         pass.visit_emphasis(&mut Emphasis::bold_level("text", 1)),
    //         Some(TransformCommand::Replace(NodeKind::Heading(Heading::new(
    //             "text", 3
    //         ))))
    //     )
    // }

    #[test]
    fn codegen_test() {
        let mut visitor = ScrapboxPrinter::new(ScrapboxPrinterConfig::default());

        let mut page = Page {
            nodes: vec![
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![
                    Node::new(NodeKind::Heading(Heading::new("heading", 1))),
                    Node::new(NodeKind::Text(Text {
                        value: "abc ".into(),
                    })),
                    Node::new(NodeKind::HashTag(HashTag {
                        value: "tag".into(),
                    })),
                    Node::new(NodeKind::Text(Text { value: " ".into() })),
                    Node::new(NodeKind::ExternalLink(ExternalLink::new(
                        Some("Rust"),
                        "https://www.rust-lang.org/",
                    ))),
                ]))),
                Node::new(NodeKind::List(List::new(vec![ListItem::new(
                    ListKind::Disc,
                    2,
                    vec![Node::new(NodeKind::Text(Text {
                        value: "abc".into(),
                    }))],
                )]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::CodeBlock(CodeBlock::new(
                        "hello.rs",
                        vec!["fn main() {", r#"    println("Hello, World!");"#, "}"],
                    )),
                )]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::Image(Image::new(
                        "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
                    )),
                )]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::Table(Table::new(
                        "table",
                        vec!["a".into(), "b".into(), "c".into()],
                        vec![vec!["d".into(), "e".into(), "f".into()]],
                    )),
                )]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::Table(Table::new(
                        "table",
                        vec!["a".into(), "b".into(), "c".into()],
                        vec![vec![]],
                    )),
                )]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::Table(Table::new("table", vec![], vec![vec![]])),
                )]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::Math(Math::new(r#"\frac{-b \pm \sqrt{b^2-4ac}}{2a}"#)),
                )]))),
            ],
        };

        let scrapbox = visitor.generate(&mut page);

        let expected = indoc! {"
            [**** heading]
            abc #tag [Rust https://www.rust-lang.org/]
            \t\t\tabc
            code:hello.rs
             fn main() {
                 println(\"Hello, World!\");
             }

            [https://www.rust-lang.org/static/images/rust-logo-blk.svg]
            table:table
             a\tb\tc
             d\te\tf

            table:table
             a\tb\tc


            [$\\frac{-b \\pm \\sqrt{b^2-4ac}}{2a}]
        "};

        assert_eq!(scrapbox, expected)
    }
}
