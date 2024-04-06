use super::super::parser_error::ParserError;
use super::ast_nodes_parser;
use super::nodes::document::DocumentNode;

use crate::jtml_lexer::JtmlToken;

use std::collections::VecDeque;

pub fn parse(tokens: &mut VecDeque<JtmlToken>) -> Result<DocumentNode, ParserError> {
    let elements = ast_nodes_parser::parse(tokens);

    // tokenを消費しきっていない場合はast_nodes_parserの最終エラーを返す(未実装)
    if !tokens.is_empty() {
        return Err(elements.1);
    }

    return Ok(DocumentNode {
        elements: elements.0,
    });
}
