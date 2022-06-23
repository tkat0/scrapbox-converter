use parser::page;
use visitor::markdown::MarkdownGen;
use wasm_bindgen::prelude::*;

mod ast;
mod parser;
mod visitor;

#[wasm_bindgen]
pub fn scrapbox_to_markdown(input: &str) -> String {
    let (_, p) = page(input).unwrap();
    let mut visitor = MarkdownGen::new();
    visitor.generate(&p)
}
