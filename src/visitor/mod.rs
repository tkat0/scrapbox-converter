use std::collections::HashMap;

use crate::ast::*;

pub mod markdown;

#[derive(Debug)]
pub enum TransformCommand {
    /// Replace the current syntax with the specified syntax.
    Replace(Syntax),
    /// Delete the current syntax.
    Delete,
}

pub trait Visitor {
    fn visit(&mut self, page: &mut Page) {
        self.visit_page(page);
    }

    fn visit_page(&mut self, page: &mut Page) {
        for line in page.lines.iter_mut() {
            self.visit_line(line);
        }
    }

    fn visit_line(&mut self, line: &mut Line) {
        // Execute all syntaxes' commands
        let mut commands = HashMap::new();
        for (i, item) in line.values.iter().enumerate() {
            let command = self.visit_syntax(item);
            if let Some(c) = command {
                commands.insert(i, c);
            }
        }

        // Replace
        for (&i, command) in &commands {
            if let TransformCommand::Replace(s) = command {
                line.values[i] = s.clone();
            }
        }

        // Delete
        let mut i = 0;
        line.values.retain(|_| {
            let retain = if let Some(TransformCommand::Delete) = commands.get(&i) {
                false
            } else {
                true
            };
            i += 1;
            retain
        });
    }

    fn visit_syntax(&mut self, syntax: &Syntax) -> Option<TransformCommand> {
        match &syntax.kind {
            SyntaxKind::HashTag(hashtag) => self.visit_hashtag(&hashtag),
            SyntaxKind::Bracket(bracket) => self.visit_bracket(&bracket),
            SyntaxKind::Text(text) => self.visit_text(&text),
        }
    }

    fn visit_hashtag(&mut self, _hashtag: &HashTag) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket(&mut self, bracket: &Bracket) -> Option<TransformCommand> {
        match &bracket.kind {
            BracketKind::InternalLink(v) => self.visit_bracket_internal_link(&v),
            BracketKind::ExternalLink(v) => self.visit_bracket_external_link(&v),
            BracketKind::Emphasis(v) => self.visit_bracket_emphasis(&v),
            BracketKind::Heading(v) => self.visit_bracket_heading(&v),
        }
    }

    fn visit_bracket_internal_link(&mut self, _link: &InternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket_external_link(&mut self, _link: &ExternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket_emphasis(&mut self, _emphasis: &Emphasis) -> Option<TransformCommand> {
        None
    }

    fn visit_bracket_heading(&mut self, _heading: &Heading) -> Option<TransformCommand> {
        None
    }

    fn visit_text(&mut self, _text: &Text) -> Option<TransformCommand> {
        None
    }
}
