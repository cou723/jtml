use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(String),
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Equal,
    Comma,
    Id(String),
}

// impl Clone for Token {
//     fn clone(&self) -> Self {
//         match self {
//             Token::Text(string) => Token::Text(string.clone()),
//             Token::LeftBracket => Token::LeftBracket,
//             Token::RightBracket => Token::RightBracket,
//             Token::LeftBrace => Token::LeftBrace,
//             Token::RightBrace => Token::RightBrace,
//             Token::Equal => Token::Equal,
//             Token::Comma => Token::Comma,
//             Token::Id(string) => Token::Id(string.clone()),
//         }
//     }
// }

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Text(_string) => write!(f, "Text({})", self),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::LeftBrace => write!(f, "LeftBrace"),
            Token::RightBrace => write!(f, "RightBrace"),
            Token::Equal => write!(f, "Equal"),
            Token::Comma => write!(f, "Comma"),
            Token::Id(_string) => write!(f, "Id({})", self),
        }
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Text,
    Id,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnexpectedToken(String),
    InternalError(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
            LexerError::InternalError(m) => write!(f, "Internal error: {}",m),
        }
    }
}

pub fn lexer(text: String) -> Result<Vec<Token>, LexerError> {
    let mut state: State = State::Other;
    let mut result: Vec<Token> = Vec::new();
    let mut buffer = String::new();
    let mut chars = text.chars();
    let mut c = chars.next();
    while c.is_some() {
        if state == State::Text {
            if c.unwrap() == '"' {
                buffer.push('"');
                result.push(Token::Text(buffer.clone()));
                state = State::Other;
                buffer.clear();
            } else if c.unwrap() == '\\' {
                match chars.next() {
                    Some(c) => {
                        if c == '"' {
                            buffer.push(c);
                        };
                    }
                    None => buffer.push('\\'),
                };
            } else {
                buffer.push(c.unwrap());
            }
        } else {
            if state == State::Id && !c.unwrap().is_alphanumeric() {
                result.push(Token::Id(buffer.clone()));
                buffer.clear();
                state = State::Other;
            }
            if c.unwrap().is_whitespace() {
                if state == State::Id {
                    result.push(Token::Id(buffer.clone()));
                    buffer.clear();
                }
                state = State::Other;
            } else {
                match c.unwrap() {
                    '{' => result.push(Token::LeftBrace),
                    '}' => result.push(Token::RightBrace),
                    '(' => result.push(Token::LeftBracket),
                    ')' => result.push(Token::RightBracket),
                    '=' => result.push(Token::Equal),
                    ',' => result.push(Token::Comma),
                    '"' => {
                        buffer.push('"');
                        state = State::Text;
                    }
                    c if c.is_alphanumeric() => {
                        buffer.push(c);
                        state = State::Id;
                    }
                    _ => {
                        return Err(LexerError::UnexpectedToken(
                            format!("invalid character {}",c.unwrap())
                        ))
                    }
                }
            }
        }
        c = chars.next();
    }
    if state == State::Id {
        result.push(Token::Id(buffer.clone()));
    }
    Ok(result)
}

#[test]
fn test_lexer() {
    assert_eq!(
        lexer("id".to_string()).unwrap(),
        vec![Token::Id("id".to_string())]
    );
    assert_eq!(
        lexer("\"text\"".to_string()).unwrap(),
        vec![Token::Text("\"text\"".to_string())]
    );
    assert_eq!(
        lexer("id1 id2".to_string()).unwrap(),
        vec![Token::Id("id1".to_string()), Token::Id("id2".to_string())]
    );
    assert_eq!(
        lexer("{id}".to_string()).unwrap(),
        vec![
            Token::LeftBrace,
            Token::Id("id".to_string()),
            Token::RightBrace
        ]
    );
    assert_eq!(
        lexer("{\"id\"}".to_string()),
        Ok(vec![
            Token::LeftBrace,
            Token::Text("\"id\"".to_string()),
            Token::RightBrace
        ])
    );
    assert_eq!(
        lexer("id1 id2 {id3}".to_string()).unwrap(),
        vec![
            Token::Id("id1".to_string()),
            Token::Id("id2".to_string()),
            Token::LeftBrace,
            Token::Id("id3".to_string()),
            Token::RightBrace
        ]
    );
    assert_eq!(
        lexer(r#"p(color="red"){"hello world!"}"#.to_string()).unwrap(),
        vec![
            Token::Id("p".to_string()),
            Token::LeftBracket,
            Token::Id("color".to_string()),
            Token::Equal,
            Token::Text("\"red\"".to_string()),
            Token::RightBracket,
            Token::LeftBrace,
            Token::Text("\"hello world!\"".to_string()),
            Token::RightBrace
        ]
    )
}
