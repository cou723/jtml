use std::{fmt, str::Chars};

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
    Neutral,
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
            LexerError::InternalError(m) => write!(f, "Internal error: {}", m),
        }
    }
}

struct LexerState {
    state: State,
    buffer: String,
    result: Vec<Token>,
}

pub fn lexer(text: String) -> Result<Vec<Token>, LexerError> {
    let mut states = LexerState {
        state: State::Neutral,
        buffer: String::new(),
        result: Vec::new(),
    };
    let mut untreated = text.chars();
    let mut c = untreated.next();
    while c.is_some() {
        if states.state == State::Text {
            // textの場合の処理
            text_lexer(c.unwrap(), &mut states, &mut untreated);
            c = untreated.next();
            continue;
        }
        // idが終了する場合は終了させる
        try_end_id(c.unwrap(), &mut states);
        if c.unwrap().is_whitespace() {
            states.state = State::Neutral;
            c = untreated.next();
            continue;
            // Other状態で読み込んでいるときの処理
        }
        neutral_character_lexer(c.unwrap(), &mut states)?;

        c = untreated.next();
    }
    if states.state == State::Id {
        states.result.push(Token::Id(states.buffer.clone()));
    }
    Ok(states.result)
}

fn text_lexer(c: char, lexer_state: &mut LexerState, untreated: &mut Chars) -> () {
    if c == '"' {
        end_text(
            &mut lexer_state.buffer,
            &mut lexer_state.state,
            &mut lexer_state.result,
        );
    } else if c == '\\' {
        match untreated.next() {
            Some(c) => {
                if c == '"' {
                    lexer_state.buffer.push(c);
                };
            }
            None => lexer_state.buffer.push('\\'),
        };
    } else {
        lexer_state.buffer.push(c);
    }
}

fn start_text(c: char, state: &mut State, buffer: &mut String) {
    buffer.push(c);
    (*state) = State::Text;
}

fn end_text(buffer: &mut String, state: &mut State, result: &mut Vec<Token>) {
    buffer.push('"');
    result.push(Token::Text(buffer.clone()));
    (*state) = State::Neutral;
    buffer.clear();
}

fn start_id(c: char, state: &mut State, buffer: &mut String) {
    buffer.push(c);
    (*state) = State::Id;
}

fn try_end_id(c: char, states: &mut LexerState) -> () {
    if states.state == State::Id && !is_id_char(c) {
        states.result.push(Token::Id(states.buffer.clone()));
        states.buffer.clear();
        states.state = State::Neutral;
    }
}

fn neutral_character_lexer(c: char, states: &mut LexerState) -> Result<(), LexerError> {
    match c {
        '{' => states.result.push(Token::LeftBrace),
        '}' => states.result.push(Token::RightBrace),
        '(' => states.result.push(Token::LeftBracket),
        ')' => states.result.push(Token::RightBracket),
        '=' => states.result.push(Token::Equal),
        ',' => states.result.push(Token::Comma),
        '"' => start_text(c, &mut states.state, &mut states.buffer),
        c if is_id_char(c) => start_id(c, &mut states.state, &mut states.buffer),
        _ => {
            return Err(LexerError::UnexpectedToken(format!(
                "invalid character {}",
                c
            )))
        }
    }
    Ok(())
}

fn is_id_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-'
}

#[test]
fn test_lexer() {
    use Token::*;
    fn lex_test(text: &str) -> Vec<Token> {
        lexer(text.to_string()).unwrap()
    }

    assert_eq!(lex_test("id"), vec![Id("id".to_string())]);
    assert_eq!(lex_test("i-d"), vec![Id("i-d".to_string())]);
    assert_eq!(lex_test("-d"), vec![Id("-d".to_string())]);
    assert_eq!(lex_test("d-"), vec![Id("d-".to_string())]);
    assert_eq!(lex_test("\"text\""), vec![Text("\"text\"".to_string())]);

    assert_eq!(
        lex_test("id1 id2"),
        vec![Id("id1".to_string()), Id("id2".to_string())]
    );

    assert_eq!(
        lex_test("{id}"),
        vec![LeftBrace, Id("id".to_string()), RightBrace]
    );

    assert_eq!(
        lex_test("{\"id\"}"),
        vec![LeftBrace, Text("\"id\"".to_string()), RightBrace]
    );

    assert_eq!(
        lex_test("id1 id2 {id3}"),
        vec![
            Id("id1".to_string()),
            Id("id2".to_string()),
            LeftBrace,
            Id("id3".to_string()),
            RightBrace
        ]
    );

    assert_eq!(
        lex_test(r#"p(color="red"){"hello world!"}"#),
        vec![
            Id("p".to_string()),
            LeftBracket,
            Id("color".to_string()),
            Equal,
            Text("\"red\"".to_string()),
            RightBracket,
            LeftBrace,
            Text("\"hello world!\"".to_string()),
            RightBrace
        ]
    )
}
