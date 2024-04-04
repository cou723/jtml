use std::collections::VecDeque;

use crate::{jtml_lexer::JtmlToken, jtml_parser::parser_error::ParserError};

pub(crate) fn parse(
    expect: JtmlToken,
    tokens: &mut VecDeque<JtmlToken>,
) -> Result<(), ParserError> {
    match tokens.remove(0) {
        Some(token) => {
            if token == expect {
                Ok(())
            } else {
                Err(ParserError::UnexpectedToken(expect, token.to_string()))
            }
        }
        None => Err(ParserError::TokenIsNotEnough(expect)),
    }
}
