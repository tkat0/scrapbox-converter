use parser::scrapbox::page;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod ast;
mod parser;
mod visitor;

use parser::Span;
use visitor::{
    markdown_printer::{MarkdownPass, MarkdownPrinter, MarkdownPrinterConfig},
    scrapbox_printer::{ScrapboxPrinter, ScrapboxPrinterConfig},
    Visitor,
};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export interface Config {
  /** Maps which bold level of Scrapbox to heading of Markdown */
  heading1Mapping: number;
  /** Maps bold of Scrapbox to the minimum level of heading of Markdown */
  boldToHeading: boolean;
}

export function scrapboxToMarkdown(input: string, config: Config): string;
"#;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub heading1_mapping: usize,
    pub bold_to_heading: bool,
}

#[wasm_bindgen(js_name = scrapboxToMarkdown, skip_typescript)]
pub fn scrapbox_to_markdown(input: &str, config: &JsValue) -> Result<String, JsError> {
    let config: Config = config.into_serde()?;
    let (_, mut p) = page(Span::new(input))?;
    let mut pass = MarkdownPass {
        h1_level: config.heading1_mapping,
        bold_to_h: config.bold_to_heading,
    };
    pass.visit(&mut p);
    let mut visitor = MarkdownPrinter::new(MarkdownPrinterConfig::default());
    Ok(visitor.generate(&mut p))
}

#[wasm_bindgen(js_name = scrapboxToAST)]
pub fn scrapbox_to_ast(input: &str) -> Result<String, JsError> {
    let (_, p) = page(Span::new(input))?;
    Ok(format!("{:#?}", &p))
}

#[wasm_bindgen(js_name = markdownToScrapbox)]
pub fn markdown_to_scrapbox(input: &str) -> Result<String, JsError> {
    let (_, mut p) = parser::markdown::page(Span::new(input))?;
    let mut visitor = ScrapboxPrinter::new(ScrapboxPrinterConfig::default());
    Ok(visitor.generate(&mut p))
}

#[wasm_bindgen(js_name = markdownToAST)]
pub fn markdown_to_ast(input: &str) -> Result<String, JsError> {
    let (_, p) = parser::markdown::page(Span::new(input))?;
    Ok(format!("{:#?}", &p))
}
