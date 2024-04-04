use crate::jtml_parser::ast_node::AstNode;

use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::parser_error::ParserError;

use std::collections::VecDeque;

use super::ast_node_parser;

pub(crate) fn parse(tokens: &mut VecDeque<JtmlToken>) -> Result<VecDeque<AstNode>, ParserError> {
    let mut elements: VecDeque<AstNode> = VecDeque::new();
    loop {
        match ast_node_parser::parse(tokens) {
            Ok(e) => {
                elements.push_back(e);
            }
            Err(_) => return Ok(elements),
        }
    }
}
