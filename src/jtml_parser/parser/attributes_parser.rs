use crate::{jtml_lexer::JtmlToken, jtml_parser::parser_error::ParserError};

use std::collections::VecDeque;

pub(crate) fn parse(
    tokens: &mut VecDeque<JtmlToken>,
) -> Result<VecDeque<(String, String)>, ParserError> {
    let mut attributes: VecDeque<(String, String)> = VecDeque::new();
    loop {
        match parse_attribute(tokens) {
            Ok((key, value)) => attributes.push_back((key, value)),
            Err(_e) => return Ok(attributes),
        }
    }
}

fn parse_attribute(tokens: &mut VecDeque<JtmlToken>) -> Result<(String, String), ParserError> {
    let key = match tokens.get(0) {
        Some(token) => match token {
            JtmlToken::Identifier(key) => key.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    JtmlToken::Identifier("".to_string()),
                    tokens[0].to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(JtmlToken::Identifier(
                "attribute key".to_string(),
            )))
        }
    };
    match tokens.get(1) {
        Some(token) => match token {
            JtmlToken::Equal => (),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    JtmlToken::Equal,
                    tokens[1].to_string(),
                ))
            }
        },
        None => return Err(ParserError::TokenIsNotEnough(JtmlToken::Equal)),
    };
    let value = match tokens.get(2) {
        Some(token) => match token {
            JtmlToken::StringLiteral(_value) => _value.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    JtmlToken::StringLiteral("string literal".to_string()),
                    tokens[2].to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(JtmlToken::StringLiteral(
                "attribute value".to_string(),
            )))
        }
    };
    tokens.pop_front();
    tokens.pop_front();
    tokens.pop_front();
    Ok((key, value))
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::{
        jtml_lexer::test_utils::lexer,
        jtml_parser::parser::attributes_parser::{self, parse_attribute},
    };

    #[test]
    fn test_attribute() {
        let mut tokens = lexer(r#"id="text""#);
        let result = parse_attribute(&mut tokens);
        assert_eq!(result.unwrap(), ("id".to_string(), "\"text\"".to_string()));
    }

    #[test]
    fn test_attributes() {
        let mut tokens = lexer(r#"id="text" id2="text2""#);
        let result = attributes_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            VecDeque::from(vec![
                ("id".to_string(), "\"text\"".to_string()),
                ("id2".to_string(), "\"text2\"".to_string())
            ])
        );
    }

    #[test]
    fn test_empty_attributes() {
        let mut tokens = lexer(r#""#);
        let result = attributes_parser::parse(&mut tokens);
        assert_eq!(result.unwrap(), VecDeque::from(vec![]));
    }
}
