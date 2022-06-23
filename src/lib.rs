use wasm_bindgen::prelude::*;

mod ast;
mod parser;
mod visitor;

#[wasm_bindgen]
pub fn scrapbox_to_markdown(input: &str) -> String {
    input.to_string()
}
