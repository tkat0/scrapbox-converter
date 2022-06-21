use crate::ast::*;

pub trait Visitor {
    fn visit_page(&mut self, page: &Page) {
        for line in &page.lines {
            self.visit_line(line);
        }
    }

    fn visit_line(&mut self, line: &Line) {
        for item in &line.items {
            self.visit_syntax(item);
        }
    }

    fn visit_syntax(&mut self, syntax: &Syntax) {
        match &syntax.kind {
            SyntaxKind::HashTag(hashtag) => self.visit_hashtag(&hashtag),
            SyntaxKind::Bracket(bracket) => self.visit_brachet(&bracket),
            SyntaxKind::Text(text) => self.visit_text(&text),
        }
    }

    fn visit_hashtag(&mut self, hashtag: &HashTag) {}
    fn visit_brachet(&mut self, bracket: &Bracket) {}
    fn visit_text(&mut self, text: &Text) {}
}

pub struct CodeGen {}

impl CodeGen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for CodeGen {
    fn visit_page(&mut self, page: &Page) {
        for line in &page.lines {
            self.visit_line(line);
            print!("\n");
        }
    }

    fn visit_hashtag(&mut self, hashtag: &HashTag) {
        print!("#{}", hashtag.value)
    }
    fn visit_brachet(&mut self, bracket: &Bracket) {
        print!("{:?}", bracket)
    }
    fn visit_text(&mut self, text: &Text) {
        print!("{}", text.value)
    }
}

mod test {
    use super::*;

    #[test]
    fn codegen_test() {
        let mut visitor = CodeGen {};

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
                ],
            }],
        };

        visitor.visit_page(&page);
    }
}
