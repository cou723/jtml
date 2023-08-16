use std::fmt::Display;
use crate::lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(lexer::Token, String),
    TokenIsNotEnough(lexer::Token),
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
