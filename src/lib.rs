use parser::page;
use serde::{Deserialize, Serialize};
use visitor::{
    markdown::{MarkdownGen, MarkdownGenConfig, MarkdownPass},
    Visitor,
};
use wasm_bindgen::prelude::*;

mod ast;
mod parser;
mod visitor;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub heading1_level_mapping: u8,
    pub bold_to_heading: bool,
}

#[wasm_bindgen(js_name = scrapboxToMarkdown)]
pub fn scrapbox_to_markdown(input: &str, config: &JsValue) -> String {
    let config: Config = config.into_serde().unwrap();
    let (_, mut p) = page(input).unwrap();
    let mut pass = MarkdownPass {
        h1_level: config.heading1_level_mapping,
        bold_to_h: config.bold_to_heading,
    };
    pass.visit(&mut p);
    let mut visitor = MarkdownGen::new(MarkdownGenConfig::default());
    visitor.generate(&mut p)
}
