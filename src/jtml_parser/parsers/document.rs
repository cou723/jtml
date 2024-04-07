use super::super::errors::ParserError;
use super::{nodes, AstRoot};

use crate::jtml_lexer::JtmlToken;

use std::collections::VecDeque;

pub fn parse(tokens: &mut VecDeque<JtmlToken>) -> Result<AstRoot, ParserError> {
    let elements = nodes::parse(tokens);

    // tokenを消費しきっていない場合はast_nodes_parserの最終エラーを返す(未実装)
    if !tokens.is_empty() {
        return Err(elements.1);
    }

    return Ok(AstRoot {
        elements: elements.0,
    });
}
