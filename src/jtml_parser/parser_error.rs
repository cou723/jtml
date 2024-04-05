use crate::jtml_lexer::{self, JtmlToken};
use std::{collections::VecDeque, error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedToken(jtml_lexer::Kind, JtmlToken, Option<VecDeque<JtmlToken>>),
    TokenIsNotEnough(Vec<jtml_lexer::Kind>),
    EmptyTokens,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(expect, actual, left_token) => {
                write!(
                    f,
                    "Unexpected token: expect {:?}, actual {} state {:?}",
                    expect, actual, left_token
                )
            }
            ParserError::TokenIsNotEnough(expect) => {
                write!(f, "Token is not enough: expect {:?}", expect)
            }
            ParserError::EmptyTokens => write!(f, "Token is empty"),
        }
    }
}

impl PartialEq for ParserError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                ParserError::UnexpectedToken(expect1, actual1, left_token1),
                ParserError::UnexpectedToken(expect2, actual2, left_token2),
            ) => {
                if left_token1.is_some() && left_token2.is_some() {
                    let left_token1 = left_token1.as_ref().unwrap();
                    let left_token2 = left_token2.as_ref().unwrap();
                    return expect1 == expect2 && actual1 == actual2 && left_token1 == left_token2;
                }
                expect1 == expect2 && actual1 == actual2
            }
            (ParserError::TokenIsNotEnough(expect1), ParserError::TokenIsNotEnough(expect2)) => {
                expect1 == expect2
            }
            (ParserError::EmptyTokens, ParserError::EmptyTokens) => true,
            _ => false,
        }
    }
}

impl Error for ParserError {}
