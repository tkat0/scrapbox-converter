use std::borrow::BorrowMut;

use crate::ast::*;

pub mod markdown_printer;
pub mod scrapbox_printer;

#[derive(Debug, PartialEq)]
pub enum TransformCommand {
    /// Replace the current node with the specified node.
    Replace(NodeKind),
    /// Delete the current node.
    Delete,
}

pub trait Visitor: Sized {
    /// if returns true, visitor doesn't walk nodes
    fn is_finish(&mut self) -> bool {
        false
    }

    fn visit(&mut self, value: &mut Page) {
        self.visit_page(value);
    }

    fn visit_page(&mut self, value: &mut Page) {
        walk_page(self, value);
    }

    fn visit_node(&mut self, value: &mut Node) {
        walk_node(self, value)
    }

    fn visit_paragraph(&mut self, value: &mut Paragraph) -> Option<TransformCommand> {
        walk_paragraph(self, value)
    }

    fn visit_list(&mut self, value: &mut List) -> Option<TransformCommand> {
        None
    }

    fn visit_hashtag(&mut self, value: &HashTag) -> Option<TransformCommand> {
        None
    }

    fn visit_internal_link(&mut self, value: &InternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_external_link(&mut self, value: &ExternalLink) -> Option<TransformCommand> {
        None
    }

    fn visit_emphasis(&mut self, value: &Emphasis) -> Option<TransformCommand> {
        None
    }

    fn visit_heading(&mut self, value: &Heading) -> Option<TransformCommand> {
        None
    }

    fn visit_block_quate(&mut self, value: &BlockQuate) -> Option<TransformCommand> {
        None
    }

    fn visit_code_block(&mut self, value: &CodeBlock) -> Option<TransformCommand> {
        None
    }

    fn visit_table(&mut self, value: &Table) -> Option<TransformCommand> {
        None
    }

    fn visit_image(&mut self, value: &Image) -> Option<TransformCommand> {
        None
    }

    fn visit_math(&mut self, value: &Math) -> Option<TransformCommand> {
        None
    }

    fn visit_text(&mut self, _text: &Text) -> Option<TransformCommand> {
        None
    }
}

pub fn walk_page<V: Visitor>(visitor: &mut V, value: &mut Page) {
    for node in value.nodes.iter_mut() {
        if visitor.is_finish() {
            return;
        }
        visitor.visit_node(node);
    }
}

pub fn walk_node<V: Visitor>(visitor: &mut V, value: &mut Node) {
    if visitor.is_finish() {
        return;
    }
    let command = match value.kind.borrow_mut() {
        NodeKind::Paragraph(v) => visitor.visit_paragraph(v),
        NodeKind::List(v) => visitor.visit_list(v),
        NodeKind::HashTag(v) => visitor.visit_hashtag(v),
        NodeKind::InternalLink(v) => visitor.visit_internal_link(v),
        NodeKind::ExternalLink(v) => visitor.visit_external_link(v),
        NodeKind::Emphasis(v) => visitor.visit_emphasis(v),
        NodeKind::Heading(v) => visitor.visit_heading(v),
        NodeKind::BlockQuate(v) => visitor.visit_block_quate(v),
        NodeKind::CodeBlock(v) => visitor.visit_code_block(v),
        NodeKind::Table(v) => visitor.visit_table(v),
        NodeKind::Image(v) => visitor.visit_image(v),
        NodeKind::Math(v) => visitor.visit_math(v),
        NodeKind::Text(v) => visitor.visit_text(v),
        NodeKind::Nop => None,
    };

    if let Some(command) = &command {
        log::debug!("command: {:?}", command);
    }

    match command {
        Some(TransformCommand::Replace(kind)) => value.kind = kind,
        Some(TransformCommand::Delete) => value.kind = NodeKind::Nop,
        None => {}
    }
}

pub fn walk_paragraph<V: Visitor>(
    visitor: &mut V,
    value: &mut Paragraph,
) -> Option<TransformCommand> {
    for node in value.children.iter_mut() {
        if visitor.is_finish() {
            return None;
        }
        visitor.visit_node(node);
    }
    None
}
