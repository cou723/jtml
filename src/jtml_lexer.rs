use logos::Logos;
use std::{
    collections::VecDeque,
    fmt::{self, Display},
};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum JtmlToken {
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),

    // #[regex(r#"/\*[^*/]*\*/"#)]
    #[regex(r#"//.*"#, |lex| {
        let comment = lex.slice()[2..].to_string();
        comment.trim_start_matches("//").trim_start_matches(' ').to_string()
    })]
    Comment(String),

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

impl Display for JtmlToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JtmlToken::StringLiteral(string) => write!(f, "Text({})", string),
            JtmlToken::Comment(string) => write!(f, "Comment({})", string),
            JtmlToken::LeftBracket => write!(f, "LeftBracket '{{'"),
            JtmlToken::RightBracket => write!(f, "RightBracket '}}'"),
            JtmlToken::LeftParen => write!(f, "LeftBrace '('"),
            JtmlToken::RightParen => write!(f, "RightBrace ')'"),
            JtmlToken::Equal => write!(f, "Equal '='"),
            JtmlToken::Identifier(_string) => write!(f, "Id({})", self),
            JtmlToken::Whitespace => write!(f, ""),
        }
    }
}

pub fn lexer(text: String) -> Result<VecDeque<JtmlToken>, String> {
    let mut result: VecDeque<JtmlToken> = VecDeque::new();
    let mut lexer = JtmlToken::lexer(text.as_str());
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

    use super::JtmlToken;

    // test 関数内で使ってるのにdead_code warningが出るため
    #[allow(dead_code)]
    fn lexer(str: &str) -> VecDeque<JtmlToken> {
        super::lexer(str.to_string()).unwrap()
    }

    #[test]
    fn string_literal() {
        let mut parsed = lexer(r#""string""#);
        assert_eq!(parsed.len(), 1);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""string""#.to_string())
        );
    }

    #[test]
    fn line_comment() {
        let parsed = lexer(r#"// comment"#);
        assert_eq!(parsed.len(), 1);
    }

    // #[test]
    // fn block_comment() {
    //     let parsed = lexer(r#"/* comment */"#);
    //     assert_eq!(parsed.len(), 0);
    // }

    #[test]
    fn attribute() {
        let mut parsed = lexer(r#"attribute = "value""#);
        assert_eq!(parsed.len(), 3);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""value""#.to_string())
        );
    }

    #[test]
    fn single_attributes() {
        let mut parsed = lexer(r#"(attribute = "value")"#);
        assert_eq!(parsed.len(), 5);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftParen);

        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""value""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightParen);
    }

    #[test]
    fn multiple_attributes() {
        let mut parsed = lexer(r#"(attribute = "value" attribute = "value")"#);
        assert_eq!(parsed.len(), 8);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftParen);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""value""#.to_string())
        );

        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""value""#.to_string())
        );

        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightParen);
    }

    #[test]
    fn value() {
        let mut parsed = lexer(r#"{"test"}"#);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftBracket);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""test""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightBracket);
    }

    #[test]
    fn empty_html_element() {
        let mut parsed = lexer(r#"p(){}"#);
        assert_eq!(parsed.len(), 5);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftParen);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftBracket);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightBracket);
    }

    #[test]
    fn string_literal_html_element() {
        let mut parsed = lexer(r#"p(){"test"}"#);
        assert_eq!(parsed.len(), 6);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftParen);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftBracket);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""test""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightBracket);
    }

    #[test]
    fn attribute_html_element() {
        let mut parsed = lexer(r#"p(attribute = "value"){}"#);
        assert_eq!(parsed.len(), 8);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftParen);

        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::Equal);

        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""value""#.to_string())
        );

        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftBracket);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightBracket);
    }

    #[test]
    fn html_element() {
        let mut parsed = lexer(r#"p(attribute = "value"){"test"}"#);
        assert_eq!(parsed.len(), 9);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("p".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftParen);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::Identifier("attribute".to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::Equal);
        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""value""#.to_string())
        );
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightParen);
        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::LeftBracket);

        assert_eq!(
            parsed.pop_front().unwrap(),
            JtmlToken::StringLiteral(r#""test""#.to_string())
        );

        assert_eq!(parsed.pop_front().unwrap(), JtmlToken::RightBracket);
    }
}

pub mod test_utils {
    use std::collections::VecDeque;

    use super::JtmlToken;

    pub fn lexer(str: &str) -> VecDeque<JtmlToken> {
        VecDeque::from(super::lexer(str.to_string()).unwrap())
    }
}
