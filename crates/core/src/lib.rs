use serde::{Deserialize, Serialize};

pub mod ast;
pub mod parser;
pub mod visitor;

use parser::markdown::IndentKind;
pub use parser::Span;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub heading1_mapping: usize,
    pub bold_to_heading: bool,
    pub indent: IndentKind,
}
