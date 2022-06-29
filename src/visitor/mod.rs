use std::collections::HashMap;

use crate::ast::*;

pub mod markdown;

#[derive(Debug)]
pub enum TransformCommand {
    /// Replace the current expr with the specified expr.
    Replace(Expr),
    /// Delete the current expr.
    Delete,
}

pub trait Visitor {
    fn visit(&mut self, value: &mut Page) {
        self.visit_page(value);
    }

    fn visit_page(&mut self, value: &mut Page) {
        for line in value.lines.iter_mut() {
            self.visit_line(line);
        }
    }

    fn visit_line(&mut self, value: &mut Line) {
        // Execute all expres' commands
        let mut commands = HashMap::new();
        for (i, item) in value.values.iter().enumerate() {
            let command = self.visit_expr(item);
            if let Some(c) = command {
                commands.insert(i, c);
            }
        }

        // Replace
        for (&i, command) in &commands {
            if let TransformCommand::Replace(s) = command {
                value.values[i] = s.clone();
            }
        }

        // Delete
        let mut i = 0;
        value.values.retain(|_| {
            let retain = if let Some(TransformCommand::Delete) = commands.get(&i) {
                false
            } else {
                true
            };
            i += 1;
            retain
        });
    }

    fn visit_expr(&mut self, value: &Expr) -> Option<TransformCommand> {
        match &value.kind {
            ExprKind::HashTag(v) => self.visit_hashtag(&v),
            ExprKind::InternalLink(v) => self.visit_internal_link(&v),
            ExprKind::ExternalLink(v) => self.visit_external_link(&v),
            ExprKind::Emphasis(v) => self.visit_emphasis(&v),
            ExprKind::Heading(v) => self.visit_heading(&v),
            ExprKind::BlockQuate(v) => self.visit_block_quate(&v),
            ExprKind::CodeBlock(v) => self.visit_code_block(&v),
            ExprKind::Table(v) => self.visit_table(&v),
            ExprKind::Image(v) => self.visit_image(&v),
            ExprKind::Math(v) => self.visit_math(&v),
            ExprKind::Text(v) => self.visit_text(&v),
        }
    }

    fn visit_hashtag(&mut self, _value: &HashTag) -> Option<TransformCommand> {
        None
    }

    fn visit_internal_link(&mut self, _value: &InternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_external_link(&mut self, _value: &ExternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_emphasis(&mut self, _value: &Emphasis) -> Option<TransformCommand> {
        None
    }

    fn visit_heading(&mut self, _value: &Heading) -> Option<TransformCommand> {
        None
    }

    fn visit_block_quate(&mut self, _value: &BlockQuate) -> Option<TransformCommand> {
        None
    }

    fn visit_code_block(&mut self, _value: &CodeBlock) -> Option<TransformCommand> {
        None
    }

    fn visit_table(&mut self, _value: &Table) -> Option<TransformCommand> {
        None
    }

    fn visit_image(&mut self, _value: &Image) -> Option<TransformCommand> {
        None
    }

    fn visit_math(&mut self, _value: &Math) -> Option<TransformCommand> {
        None
    }

    fn visit_text(&mut self, _text: &Text) -> Option<TransformCommand> {
        None
    }
}
