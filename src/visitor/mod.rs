use crate::ast::*;

pub mod markdown;

#[derive(Debug, PartialEq)]
pub enum TransformCommand {
    /// Replace the current node with the specified node.
    Replace(NodeKind),
    /// Delete the current node.
    Delete,
}

pub trait Visitor: Sized {
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

fn walk_page<V: Visitor>(visitor: &mut V, value: &mut Page) {
    for node in value.nodes.iter_mut() {
        visitor.visit_node(node);
    }
}

fn walk_node<V: Visitor>(visitor: &mut V, value: &mut Node) {
    let command = match value.kind.clone() {
        NodeKind::Paragraph(mut v) => visitor.visit_paragraph(&mut v),
        NodeKind::List(mut v) => visitor.visit_list(&mut v),
        NodeKind::HashTag(v) => visitor.visit_hashtag(&v),
        NodeKind::InternalLink(v) => visitor.visit_internal_link(&v),
        NodeKind::ExternalLink(v) => visitor.visit_external_link(&v),
        NodeKind::Emphasis(v) => visitor.visit_emphasis(&v),
        NodeKind::Heading(v) => visitor.visit_heading(&v),
        NodeKind::BlockQuate(v) => visitor.visit_block_quate(&v),
        NodeKind::CodeBlock(v) => visitor.visit_code_block(&v),
        NodeKind::Table(v) => visitor.visit_table(&v),
        NodeKind::Image(v) => visitor.visit_image(&v),
        NodeKind::Math(v) => visitor.visit_math(&v),
        NodeKind::Text(v) => visitor.visit_text(&v),
        NodeKind::Nop => None,
    };

    match command {
        Some(TransformCommand::Replace(kind)) => value.kind = kind,
        Some(TransformCommand::Delete) => value.kind = NodeKind::Nop,
        None => {}
    }
}

fn walk_paragraph<V: Visitor>(visitor: &mut V, value: &mut Paragraph) -> Option<TransformCommand> {
    for node in value.children.iter_mut() {
        visitor.visit_node(node);
    }
    None
}
