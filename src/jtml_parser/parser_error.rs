use crate::jtml_lexer;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(jtml_lexer::JtmlToken, String),
    TokenIsNotEnough(jtml_lexer::JtmlToken),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(expect, actual) => {
                write!(f, "Unexpected token: expect {}, actual {}", expect, actual)
            }
            ParserError::TokenIsNotEnough(expect) => {
                write!(f, "Token is not enough: expect {}", expect)
            }
        }
    }
}

impl Error for ParserError {}
