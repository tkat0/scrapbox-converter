use parser::{
    markdown::{IndentKind, MarkdownParserConfig, MarkdownParserContext},
    scrapbox::page,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod ast;
pub mod parser;
pub mod visitor;

pub use parser::Span;
use visitor::{
    markdown_printer::{MarkdownPass, MarkdownPrinter, MarkdownPrinterConfig},
    scrapbox_printer::{ScrapboxPrinter, ScrapboxPrinterConfig},
    Visitor,
};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type IndentKind = {type: "Tab"} | {type: "Space", size: number};

export interface Config {
  /** Maps which bold level of Scrapbox to heading of Markdown */
  heading1Mapping: number;
  /** Maps bold of Scrapbox to the minimum level of heading of Markdown */
  boldToHeading: boolean;
  /** indent of markdown list */
  indent: IndentKind;
}

export function scrapboxToMarkdown(input: string, config: Config): string;
export function scrapboxToAST(input: string, config: Config): string;
export function markdownToScrapbox(input: string, config: Config): string;
export function markdownToAST(input: string, config: Config): string;
"#;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub heading1_mapping: usize,
    pub bold_to_heading: bool,
    pub indent: IndentKind,
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

#[wasm_bindgen(js_name = scrapboxToAST, skip_typescript)]
pub fn scrapbox_to_ast(input: &str, config: &JsValue) -> Result<String, JsError> {
    let _config: Config = config.into_serde()?;
    let (_, p) = page(Span::new(input))?;
    Ok(format!("{:#?}", &p))
}

#[wasm_bindgen(js_name = markdownToScrapbox, skip_typescript)]
pub fn markdown_to_scrapbox(input: &str, config: &JsValue) -> Result<String, JsError> {
    let config: Config = config.into_serde()?;
    let context = MarkdownParserContext {
        config: MarkdownParserConfig {
            indent: config.indent,
        },
    };
    let (_, mut p) = parser::markdown::page(Span::new_extra(input, context))?;
    let mut visitor = ScrapboxPrinter::new(ScrapboxPrinterConfig::default());
    Ok(visitor.generate(&mut p))
}

#[wasm_bindgen(js_name = markdownToAST, skip_typescript)]
pub fn markdown_to_ast(input: &str, config: &JsValue) -> Result<String, JsError> {
    let config: Config = config.into_serde()?;
    let context = MarkdownParserContext {
        config: MarkdownParserConfig {
            indent: config.indent,
        },
    };
    let context = MarkdownParserContext::default();
    let (_, p) = parser::markdown::page(Span::new_extra(input, context))?;
    Ok(format!("{:#?}", &p))
}
