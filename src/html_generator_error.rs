use std::fmt::{Display, Formatter, Result};

use crate::{jtml_lexer::LexerError, jtml_parser::parser_error::ParserError};

#[derive(Debug, PartialEq)]
pub enum HtmlGeneratorError {
    ParseError(ParserError),
    LexerError(LexerError),
}

impl Display for HtmlGeneratorError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            HtmlGeneratorError::ParseError(e) => write!(f, "{:?}", e),
            HtmlGeneratorError::LexerError(e) => write!(f, "{:?}", e),
        }
    }
}
