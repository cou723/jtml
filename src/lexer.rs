use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Display)]
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
#[derive(PartialEq, Debug)]
enum State {
    Text,
    Id,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
enum LexerError {
    UnexpectedToken(String),
}
pub fn lexer(text: String) -> Result<Vec<Token>, LexerError> {
    let mut state: State = State::Other;
    let mut result: Vec<Token> = Vec::new();
    let mut buffer = String::new();
    let mut chars = text.chars();
    let mut c = chars.next();
    while c.is_some() {
        if state != State::Text {
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
                if buffer.len() != 0 {
                    eprintln!("buffer内に文字({})が残っています", buffer);
                    buffer.clear();
                };
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
                    _ => eprintln!("invalid character {}", c.unwrap()),
                }
            }
        } else {
                if c.unwrap() == '"' {
                    buffer.push('"');
                    result.push(Token::Text(buffer.clone()));
                    state = State::Other;
                    buffer.clear();
                }
                else if c.unwrap() == '\\' {
                    match chars.next() {
                        Some(c) => {
                            if c == '"' {
                                buffer.push(c);
                            };
                        }
                        None => buffer.push('\\'),
                    };
                }else{
                    buffer.push(c.unwrap());
                }
        }
        c = chars.next();
    }
    if state==State::Id {
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
        lexer("p(color=\"red\"){\"hello world!\"}".to_string()).unwrap(),
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
