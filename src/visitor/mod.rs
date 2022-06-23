use crate::ast::*;

pub mod markdown;

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
            SyntaxKind::Bracket(bracket) => self.visit_bracket(&bracket),
            SyntaxKind::Text(text) => self.visit_text(&text),
        }
    }

    fn visit_hashtag(&mut self, _hashtag: &HashTag) {}

    fn visit_bracket(&mut self, bracket: &Bracket) {
        match &bracket.kind {
            BracketKind::InternalLink(v) => self.visit_bracket_internal_link(&v),
            BracketKind::ExternalLink(v) => self.visit_bracket_external_link(&v),
            BracketKind::Decoration(v) => self.visit_bracket_decoration(&v),
        }
    }

    fn visit_bracket_internal_link(&mut self, _link: &InternalLink) {}

    fn visit_bracket_external_link(&mut self, _link: &ExternalLink) {}

    fn visit_bracket_decoration(&mut self, _decoration: &Decoration) {}

    fn visit_text(&mut self, _text: &Text) {}
}
