use parser::page;
use visitor::{
    markdown::{MarkdownGen, MarkdownPass},
    Visitor,
};
use wasm_bindgen::prelude::*;

mod ast;
mod parser;
mod visitor;

#[wasm_bindgen]
pub fn scrapbox_to_markdown(input: &str) -> String {
    let (_, mut p) = page(input).unwrap();
    let mut pass = MarkdownPass {
        h1_level: 3,
        bold_to_h: true,
    };
    pass.visit(&mut p);
    let mut visitor = MarkdownGen::new();
    visitor.generate(&mut p)
}
