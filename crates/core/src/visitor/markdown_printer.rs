use super::{walk_paragraph, TransformCommand, Visitor};
use crate::ast::*;

pub struct MarkdownPass {
    // Examples:
    // - `h1_level1` == 3: [*** text] -> `# text`
    // - `h1_level1` == 3: [** text] -> `## text`
    // - `h1_level1` == 3: [* text] -> `### text` or `**text**` (see `bold_to_h`)
    // - `h1_level1` == 5: [***** text] -> `# text`
    pub h1_level: usize,
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
    fn visit_emphasis(&mut self, emphasis: &Emphasis) -> Option<TransformCommand> {
        let h_level = (self.h1_level + 1).saturating_sub(emphasis.bold);
        if 0 < h_level
            && h_level <= self.h1_level
            && (self.bold_to_h || (!self.bold_to_h && emphasis.bold > 1))
        {
            Some(TransformCommand::Replace(NodeKind::Heading(Heading::new(
                &emphasis.text,
                h_level,
            ))))
        } else {
            None
        }
    }

    fn visit_list(&mut self, value: &mut List) -> Option<TransformCommand> {
        let mut new_nodes: Vec<Node> = vec![];
        let mut last_is_code_block = true;
        for item in value.children.iter() {
            if let Some(NodeKind::CodeBlock(code_block)) = &item.children.get(0).map(|c| &c.kind) {
                new_nodes.push(Node::new(NodeKind::CodeBlock(code_block.clone())));
                last_is_code_block = true;

                if item.children.len() > 1 {
                    let children: Vec<Node> = item.children.clone().into_iter().skip(1).collect();
                    new_nodes.push(Node::new(NodeKind::List(List::new(vec![ListItem::new(
                        item.kind.clone(),
                        item.level,
                        children,
                    )]))))
                }
            } else {
                if last_is_code_block {
                    new_nodes.push(Node::new(NodeKind::List(List::new(vec![item.clone()]))));
                } else {
                    if let NodeKind::List(list) = &mut new_nodes.last_mut().unwrap().kind {
                        list.children.push(item.clone());
                    }
                }
                last_is_code_block = false;
            }
        }

        if new_nodes.len() == 1 {
            Some(TransformCommand::Replace(new_nodes[0].kind.clone()))
        } else {
            // TODO: [refactor] create a kind of Group instead of Paragraph
            Some(TransformCommand::Replace(NodeKind::Paragraph(
                Paragraph::new(new_nodes),
            )))
        }
    }
}

pub struct MarkdownPrinterConfig {
    pub indent: String,
}

impl Default for MarkdownPrinterConfig {
    fn default() -> Self {
        Self {
            indent: "  ".into(),
        }
    }
}

pub struct MarkdownPrinter {
    document: String,
    config: MarkdownPrinterConfig,
}

impl MarkdownPrinter {
    pub fn new(config: MarkdownPrinterConfig) -> Self {
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

impl Visitor for MarkdownPrinter {
    fn visit_paragraph(&mut self, value: &mut Paragraph) -> Option<TransformCommand> {
        walk_paragraph(self, value);
        self.document.push_str("\n");
        None
    }

    fn visit_list(&mut self, value: &mut List) -> Option<TransformCommand> {
        for item in value.children.iter_mut() {
            let indent = self.config.indent.repeat(item.level - 1);
            match &item.kind {
                ListKind::Disc => self.document.push_str(&format!("{}* ", indent)),
                ListKind::Decimal => self.document.push_str(&format!("{}1. ", indent)),
                _ => {}
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
        self.document.push_str(&format!("[[{}]]", value.title));
        None
    }

    fn visit_external_link(&mut self, value: &ExternalLink) -> Option<TransformCommand> {
        if let Some(title) = &value.title {
            self.document
                .push_str(&format!("[{}]({})", title, value.url));
        } else {
            if value.url.starts_with("https://gyazo.com/") {
                self.document
                    .push_str(&format!("![]({}/max_size/400)", value.url));
            } else {
                self.document.push_str(&format!("{}", value.url));
            }
        }
        None
    }

    fn visit_emphasis(&mut self, value: &Emphasis) -> Option<TransformCommand> {
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

    fn visit_heading(&mut self, value: &Heading) -> Option<TransformCommand> {
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
        for code in &value.children {
            self.document.push_str(&format!("{}\n", code));
        }
        self.document.push_str("```\n");
        None
    }

    fn visit_table(&mut self, value: &Table) -> Option<TransformCommand> {
        if value.header.is_empty() {
            return None;
        }

        self.document
            .push_str(&format!("| {} |", value.header.join(" | ")));
        self.document.push_str("\n");

        let sep = vec!["---"];
        self.document.push_str(&format!(
            "| {} |",
            sep.repeat(value.header.len()).join(" | ")
        ));

        self.document.push_str("\n");
        for row in &value.rows {
            if row.is_empty() {
                break;
            }
            self.document.push_str(&format!("| {} |", row.join(" | ")));
            self.document.push_str("\n");
        }
        None
    }

    fn visit_image(&mut self, value: &Image) -> Option<TransformCommand> {
        self.document.push_str(&format!("![]({})", value.uri));
        None
    }

    fn visit_math(&mut self, value: &Math) -> Option<TransformCommand> {
        self.document.push_str(&format!("$${}$$", value.value));
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

    #[test]
    fn pass_test() {
        let mut pass = MarkdownPass::default();

        assert_eq!(
            pass.visit_emphasis(&mut Emphasis::bold_level("text", 3)),
            Some(TransformCommand::Replace(NodeKind::Heading(Heading::new(
                "text", 1
            ))))
        );
    }

    #[test]
    fn pass_bold_to_h_test() {
        let mut pass = MarkdownPass {
            h1_level: 3,
            bold_to_h: true,
        };

        // TODO(tkat0): not supoprted: `[*-/ mix]` -> `### *~~mix~~*` (but `### mix`)
        assert_eq!(
            pass.visit_emphasis(&mut Emphasis::bold_level("text", 1)),
            Some(TransformCommand::Replace(NodeKind::Heading(Heading::new(
                "text", 3
            ))))
        )
    }

    #[test]
    fn pass_flatten_code_block_in_list() {
        let mut pass = MarkdownPass::default();

        let code_block = Node::new(NodeKind::CodeBlock(CodeBlock::new(
            "hello.rs",
            vec!["fn main() {", r#"    println("Hello, World!");"#, "}"],
        )));

        let mut input = List::new(vec![ListItem::new(
            ListKind::Disc,
            1,
            vec![code_block.clone()],
        )]);

        assert_eq!(
            pass.visit_list(&mut input),
            Some(TransformCommand::Replace(code_block.kind))
        )
    }

    #[test]
    fn codegen_test() {
        let mut visitor = MarkdownPrinter::new(MarkdownPrinterConfig::default());

        // TODO(tkat0): move this example to ast.rs and reuse for each printer test.
        let mut page = Page {
            nodes: vec![
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::Heading(Heading::new("heading", 1)),
                )]))),
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![
                    Node::new(NodeKind::Text(Text::new("abc "))),
                    Node::new(NodeKind::HashTag(HashTag::new("tag"))),
                    Node::new(NodeKind::Text(Text::new(" "))),
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
                Node::new(NodeKind::Paragraph(Paragraph::new(vec![Node::new(
                    NodeKind::ExternalLink(ExternalLink::new(
                        None,
                        "https://gyazo.com/5f93e65a3b979ae5333aca4f32600611",
                    )),
                )]))),
            ],
        };

        let markdown = visitor.generate(&mut page);

        let expected = indoc! {r#"
            # heading
            abc #tag [Rust](https://www.rust-lang.org/)
              * abc
            ```hello.rs
            fn main() {
                println("Hello, World!");
            }
            ```

            ![](https://www.rust-lang.org/static/images/rust-logo-blk.svg)
            | a | b | c |
            | --- | --- | --- |
            | d | e | f |

            | a | b | c |
            | --- | --- | --- |


            $$\frac{-b \pm \sqrt{b^2-4ac}}{2a}$$
            ![](https://gyazo.com/5f93e65a3b979ae5333aca4f32600611/max_size/400)
        "#};

        assert_eq!(markdown, expected)
    }
}
