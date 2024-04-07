use std::fmt::{Display, Formatter, Result};

use crate::{jtml_lexer::LexerError, jtml_parser::ParserError};

#[derive(Debug, PartialEq)]
pub enum HtmlConverterError {
    ParseError(ParserError),
    LexerError(LexerError),
}

impl Display for HtmlConverterError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            HtmlConverterError::ParseError(e) => write!(f, "{:?}", e),
            HtmlConverterError::LexerError(e) => write!(f, "{:?}", e),
        }
    }
}
