use parser::page;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod ast;
mod parser;
mod visitor;

use parser::Span;
use visitor::{
    markdown::{MarkdownGen, MarkdownGenConfig, MarkdownPass},
    Visitor,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub heading1_mapping: u8,
    pub bold_to_heading: bool,
}

#[wasm_bindgen(js_name = scrapboxToMarkdown)]
pub fn scrapbox_to_markdown(input: &str, config: &JsValue) -> Result<String, JsError> {
    let config: Config = config.into_serde()?;
    let (_, mut p) = page(Span::new(input))?;
    let mut pass = MarkdownPass {
        h1_level: config.heading1_mapping,
        bold_to_h: config.bold_to_heading,
    };
    pass.visit(&mut p);
    let mut visitor = MarkdownGen::new(MarkdownGenConfig::default());
    Ok(visitor.generate(&mut p))
}

#[wasm_bindgen(js_name = scrapboxToAST)]
pub fn scrapbox_to_ast(input: &str) -> Result<String, JsError> {
    let (_, p) = page(Span::new(input))?;
    Ok(format!("{:#?}", &p))
}
