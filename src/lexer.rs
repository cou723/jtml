use logos::Logos;
use std::{fmt, str::Chars};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),

    #[regex(r#"[0-9A-Za-z]+"#, |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("(")]
    LeftBracket,

    #[token(")")]
    RightBracket,

    #[token("=")]
    Equal,

    #[regex(r"\s+", logos::skip)]
    Whitespace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::StringLiteral(_string) => write!(f, "Text({})", _string),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::LeftBrace => write!(f, "LeftBrace"),
            Token::RightBrace => write!(f, "RightBrace"),
            Token::Equal => write!(f, "Equal"),
            Token::Identifier(_string) => write!(f, "Id({})", self),
            Token::Whitespace => write!(f, ""),
        }
    }
}

pub fn lexer(text: String) -> Vec<Result<Token, ()>> {
    let mut result: Vec<Result<Token, ()>> = Vec::new();
    let mut lexer = Token::lexer(text.as_str());
    while let Some(token) = lexer.next() {
        result.push(token);
    }
    return result;
}
