use super::super::parser_error::ParserError;
use super::ast_nodes_parser;

use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::document;

use std::collections::VecDeque;

pub fn parser(tokens: &mut VecDeque<JtmlToken>) -> Result<document::Document, ParserError> {
    Ok(document::Document {
        elements: ast_nodes_parser::parse(tokens)?,
    })
}
