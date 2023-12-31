pub mod child_element;
mod document;
pub mod element;
mod parser_error;

use crate::lexer;
pub use child_element::Child;
pub use document::Document;
pub use element::Element;
use parser_error::ParserError;
use std::collections::VecDeque;

pub fn parser(tokens: &mut VecDeque<lexer::Token>) -> Result<Document, ParserError> {
    document(tokens)
}

fn document(tokens: &mut VecDeque<lexer::Token>) -> Result<Document, ParserError> {
    Ok(Document {
        elements: elements(tokens)?,
    })
}

fn elements(tokens: &mut VecDeque<lexer::Token>) -> Result<VecDeque<Child>, ParserError> {
    let mut elements: VecDeque<Child> = VecDeque::new();
    loop {
        match element(tokens) {
            Ok(element) => {
                elements.push_back(element);
            }
            Err(_e) => return Ok(elements),
        }
    }
}

fn element(tokens: &mut VecDeque<lexer::Token>) -> Result<Child, ParserError> {
    let front_token = tokens.front();
    match front_token {
        Some(token) => match token {
            lexer::Token::StringLiteral(_text) => {
                let text = _text.clone();
                tokens.pop_front();
                return Ok(Child::Text(text));
            }
            _ => (),
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::Identifier(
                "element-name".to_string(),
            )))
        }
    };
    let element_name = match tokens.front() {
        Some(token) => match token {
            lexer::Token::Identifier(_id) => _id.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    lexer::Token::Identifier("".to_string()),
                    token.to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::Identifier(
                "element-name".to_string(),
            )))
        }
    };
    tokens.pop_front();

    one_token(lexer::Token::LeftParen, tokens)?;
    let attributes = attributes(tokens)?;
    one_token(lexer::Token::RightParen, tokens)?;

    one_token(lexer::Token::LeftBracket, tokens)?;
    let children = elements(tokens)?;
    one_token(lexer::Token::RightBracket, tokens)?;

    Ok(Child::Element(Element {
        element_name: element_name,
        attributes: attributes,
        children: children,
    }))
}

fn attributes(
    tokens: &mut VecDeque<lexer::Token>,
) -> Result<VecDeque<(String, String)>, ParserError> {
    let mut attributes: VecDeque<(String, String)> = VecDeque::new();
    loop {
        match attribute(tokens) {
            Ok((key, value)) => attributes.push_back((key, value)),
            Err(_e) => return Ok(attributes),
        }
    }
}

fn attribute(tokens: &mut VecDeque<lexer::Token>) -> Result<(String, String), ParserError> {
    let key = match tokens.get(0) {
        Some(token) => match token {
            lexer::Token::Identifier(_key) => _key.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    lexer::Token::Identifier("any".to_string()),
                    tokens[0].to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::Identifier(
                "attribute key".to_string(),
            )))
        }
    };
    match tokens.get(1) {
        Some(token) => match token {
            lexer::Token::Equal => (),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    lexer::Token::Equal,
                    tokens[1].to_string(),
                ))
            }
        },
        None => return Err(ParserError::TokenIsNotEnough(lexer::Token::Equal)),
    };
    let value = match tokens.get(2) {
        Some(token) => match token {
            lexer::Token::StringLiteral(_value) => _value.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    lexer::Token::StringLiteral("any".to_string()),
                    tokens[2].to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::StringLiteral(
                "attribute value".to_string(),
            )))
        }
    };
    tokens.pop_front();
    tokens.pop_front();
    tokens.pop_front();
    Ok((key, value))
}

fn one_token(expect: lexer::Token, tokens: &mut VecDeque<lexer::Token>) -> Result<(), ParserError> {
    match tokens.remove(0) {
        Some(token) => {
            if token == expect {
                Ok(())
            } else {
                Err(ParserError::UnexpectedToken(expect, token.to_string()))
            }
        }
        None => Err(ParserError::TokenIsNotEnough(expect)),
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::{
        lexer::{self, Token},
        parser::{self, Element},
    };

    fn lexer(str: &str) -> VecDeque<Token> {
        VecDeque::from(lexer::lexer(str.to_string()).unwrap())
    }

    #[test]
    fn test_attribute() {
        let mut tokens = lexer(r#"id="text""#);
        let result = parser::attribute(&mut tokens);
        assert_eq!(result.unwrap(), ("id".to_string(), "\"text\"".to_string()));
    }

    #[test]
    fn test_attributes() {
        let mut tokens = lexer(r#"id="text" id2="text2""#);
        let result = parser::attributes(&mut tokens);
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
        let result = parser::attributes(&mut tokens);
        assert_eq!(result.unwrap(), VecDeque::from(vec![]));
    }

    #[test]
    fn test_element() {
        let mut tokens = lexer(r#"p(){}"#);
        let result = parser::element(&mut tokens);
        assert_eq!(
            result.unwrap(),
            parser::Child::Element(Element {
                element_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![])
            })
        );
    }

    #[test]
    fn test_element_with_attribute() {
        let mut tokens = lexer(r#"p(width="100"){}"#);
        let result = parser::element(&mut tokens);
        assert_eq!(
            result.unwrap(),
            parser::Child::Element(Element {
                element_name: "p".to_string(),
                attributes: VecDeque::from(vec![("width".to_string(), r#""100""#.to_string())]),
                children: VecDeque::from(vec![])
            })
        );
    }
    #[test]
    fn test_element_with_content() {
        let mut tokens = lexer(r#"p(){"hello"}"#);
        let result = parser::element(&mut tokens);

        assert_eq!(
            result.unwrap(),
            parser::Child::Element(Element {
                element_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![parser::Child::Text("\"hello\"".to_string())])
            })
        );
    }

    #[test]
    fn test_element_with_contents() {
        let mut tokens = lexer(r#"p(){"hello""world"}"#);
        let result = parser::element(&mut tokens);
        assert_eq!(
            result.unwrap(),
            parser::Child::Element(Element {
                element_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![
                    parser::Child::Text("\"hello\"".to_string()),
                    parser::Child::Text("\"world\"".to_string())
                ])
            })
        );
    }

    #[test]
    fn test_element_with_child_element() {
        let mut tokens = lexer(r#"p(){p(){"test"}p(){"test1""test2"}}}"#);
        let result = parser::element(&mut tokens);

        assert_eq!(
            result.unwrap(),
            parser::Child::Element(Element {
                element_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![
                    parser::Child::Element(Element {
                        element_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![parser::Child::Text("\"test\"".to_string())])
                    }),
                    parser::Child::Element(Element {
                        element_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![
                            parser::Child::Text("\"test1\"".to_string()),
                            parser::Child::Text("\"test2\"".to_string())
                        ])
                    })
                ])
            })
        );
    }

    #[test]
    fn test_document() {
        let mut tokens = lexer(r#"h1(){}p(){}"#);
        let result = parser::document(&mut tokens);

        assert_eq!(
            result.unwrap(),
            parser::Document {
                elements: VecDeque::from(vec![
                    parser::Child::Element(Element {
                        element_name: "h1".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    }),
                    parser::Child::Element(Element {
                        element_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    })
                ])
            }
        );
    }
}
