use logos::Logos;
use std::{collections::VecDeque, fmt};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),

    #[regex(r#"/\*[^*/]*\*/"#, logos::skip)]
    #[regex(r#"//.*"#, logos::skip)]
    Comment,

    #[regex(r#"[0-9A-Za-z\-]+"#, |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("{")]
    LeftBracket,

    #[token("}")]
    RightBracket,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("=")]
    Equal,

    #[regex(r"\s+", logos::skip)]
    Whitespace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::StringLiteral(_string) => write!(f, "Text({})", _string),
            Token::Comment => write!(f, "Comment"),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::LeftParen => write!(f, "LeftBrace"),
            Token::RightParen => write!(f, "RightBrace"),
            Token::Equal => write!(f, "Equal"),
            Token::Identifier(_string) => write!(f, "Id({})", self),
            Token::Whitespace => write!(f, ""),
        }
    }
}

pub fn lexer(text: String) -> Result<VecDeque<Token>, String> {
    let mut result: VecDeque<Token> = VecDeque::new();
    let mut lexer = Token::lexer(text.as_str());
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => result.push_back(token),
            Err(_) => return Err(format!("Invalid token: {}", lexer.slice())),
        }
    }
    return Ok(result);
}

mod test {
    use std::collections::VecDeque;

    use super::Token;

    // test 関数内で使ってるのになぜかdead_code warningが出るため
    #[allow(dead_code)]
    fn lexer(str: &str) -> VecDeque<Token> {
        super::lexer(str.to_string()).unwrap()
    }

    #[test]
    fn string_literal() {
        let mut parsed = lexer(r#""string""#);
        assert_eq!(parsed.len(), 1);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""string""#.to_string())
        );
    }

    #[test]
    fn line_comment() {
        let parsed = lexer(r#"// comment"#);
        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn block_comment() {
        let parsed = lexer(r#"/* comment */"#);
        assert_eq!(parsed.len(), 0);
    }

    #[test]
    fn attribute() {
        let mut parsed = lexer(r#"attribute = "value""#);
        assert_eq!(parsed.len(), 3);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""value""#.to_string())
        );
    }

    #[test]
    fn single_attributes() {
        let mut parsed = lexer(r#"(attribute = "value")"#);
        assert_eq!(parsed.len(), 5);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftParen);

        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""value""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightParen);
    }

    #[test]
    fn multiple_attributes() {
        let mut parsed = lexer(r#"(attribute = "value" attribute = "value")"#);
        assert_eq!(parsed.len(), 8);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftParen);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""value""#.to_string())
        );

        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""value""#.to_string())
        );

        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightParen);
    }

    #[test]
    fn value() {
        let mut parsed = lexer(r#"{"test"}"#);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftBracket);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""test""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightBracket);
    }

    #[test]
    fn empty_html_element() {
        let mut parsed = lexer(r#"p(){}"#);
        assert_eq!(parsed.len(), 5);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftParen);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftBracket);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightBracket);
    }

    #[test]
    fn string_literal_html_element() {
        let mut parsed = lexer(r#"p(){"test"}"#);
        assert_eq!(parsed.len(), 6);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftParen);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftBracket);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""test""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightBracket);
    }

    #[test]
    fn attribute_html_element() {
        let mut parsed = lexer(r#"p(attribute = "value"){}"#);
        assert_eq!(parsed.len(), 8);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftParen);

        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::Equal);

        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""value""#.to_string())
        );

        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftBracket);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightBracket);
    }

    #[test]
    fn html_element() {
        let mut parsed = lexer(r#"p(attribute = "value"){"test"}"#);
        assert_eq!(parsed.len(), 9);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftParen);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""value""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), super::Token::LeftBracket);

        assert_eq!(
            parsed.pop_front().unwrap(),
            super::Token::StringLiteral(r#""test""#.to_string())
        );

        assert_eq!(parsed.pop_front().unwrap(), super::Token::RightBracket);
    }
}
