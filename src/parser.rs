use std::{collections::VecDeque, fmt::Display};

use crate::lexer;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    elements: VecDeque<Child>,
}

impl Document {
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        for element in &self.elements {
            html.push_str(&element.to_html());
        }
        html
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(lexer::Token, String),
    TokenIsNotEnough(lexer::Token),
}

impl Display for ParserError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self{
            ParserError::UnexpectedToken(expect, actual) => write!(f, "Unexpected token: expect {}, actual {}", expect, actual),
            ParserError::TokenIsNotEnough(expect) => write!(f, "Token is not enough: expect {}", expect),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Element {
    element_name: String,
    attributes: VecDeque<(String, String)>,
    children: VecDeque<Child>,
}

impl Element{
    fn to_html(&self) -> String{
        let mut html = String::new();
        html.push_str(&format!("<{}", self.element_name));
        for (key, value) in &self.attributes{
            html.push_str(&format!(" {}={}", key, value));
        }
        html.push_str(">");
        for child in &self.children{
            html.push_str(&child.to_html());
        }
        html.push_str(&format!("</{}>", self.element_name));
        html
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Child {
    Element(Element),
    Text(String),
}

impl Child{
    fn to_html(&self)->String{
        match self{
            Child::Element(element) => element.to_html(),
            Child::Text(text) => text.clone().trim_matches('"').to_string(),
        }
    }
}

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
            lexer::Token::Text(_text) => {
                let text = _text.clone();
                tokens.pop_front();
                return Ok(Child::Text(text));
            }
            _ => (),
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::Id(
                "element-name".to_string(),
            )))
        }
    };
    let element_name = match tokens.front() {
        Some(token) => match token {
            lexer::Token::Id(_id) => _id.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    lexer::Token::Id("".to_string()),
                    token.to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::Id(
                "element-name".to_string(),
            )))
        }
    };
    tokens.pop_front();

    one_token(lexer::Token::LeftBracket, tokens)?;
    let attributes = attributes(tokens)?;
    one_token(lexer::Token::RightBracket, tokens)?;

    one_token(lexer::Token::LeftBrace, tokens)?;
    let children = elements(tokens)?;
    one_token(lexer::Token::RightBrace, tokens)?;

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
            lexer::Token::Id(_key) => _key.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    lexer::Token::Id("any".to_string()),
                    tokens[0].to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::Id(
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
            lexer::Token::Text(_value) => _value.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    lexer::Token::Text("any".to_string()),
                    tokens[2].to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(lexer::Token::Text(
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
        Some(token) => match token {
            _expect => Ok(()),
            _ => Err(ParserError::UnexpectedToken(expect, token.to_string())),
        },
        None => Err(ParserError::TokenIsNotEnough(expect)),
    }
}

mod test {
    use std::collections::VecDeque;

    use crate::lexer;
    use crate::parser;
    use crate::parser::Element;
    #[test]
    fn test_attribute() {
        let mut tokens = VecDeque::from(vec![
            lexer::Token::Id("id".to_string()),
            lexer::Token::Equal,
            lexer::Token::Text("text".to_string()),
        ]);
        let result = parser::attribute(&mut tokens);
        assert_eq!(result.unwrap(), ("id".to_string(), "text".to_string()));
    }

    #[test]
    fn test_attributes() {
        let mut tokens = VecDeque::from(vec![
            lexer::Token::Id("id".to_string()),
            lexer::Token::Equal,
            lexer::Token::Text("text".to_string()),
            lexer::Token::Id("id2".to_string()),
            lexer::Token::Equal,
            lexer::Token::Text("text2".to_string()),
        ]);
        let result = parser::attributes(&mut tokens);
        assert_eq!(
            result.unwrap(),
            VecDeque::from(vec![
                ("id".to_string(), "text".to_string()),
                ("id2".to_string(), "text2".to_string())
            ])
        );
    }

    #[test]
    fn test_empty_attributes() {
        let mut tokens = VecDeque::from(vec![]);
        let result = parser::attributes(&mut tokens);
        assert_eq!(result.unwrap(), VecDeque::from(vec![]));
    }

    #[test]
    fn test_element() {
        let mut tokens = VecDeque::from(lexer::lexer(r#"p(){}"#.to_string()).unwrap());
        println!("{:?}", tokens);
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
        let mut tokens = VecDeque::from(lexer::lexer(r#"p(width="100"){}"#.to_string()).unwrap());
        println!("{:?}", tokens);
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
        let mut tokens = VecDeque::from(lexer::lexer(r#"p(){"hello"}"#.to_string()).unwrap());
        println!("{:?}", tokens);
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
        let mut tokens =
            VecDeque::from(lexer::lexer(r#"p(){"hello""world"}"#.to_string()).unwrap());
        println!("{:?}", tokens);
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
        let mut tokens = VecDeque::from(
            lexer::lexer(r#"p(){p(){"test"}p(){"test1""test2"}}}"#.to_string()).unwrap(),
        );
        println!("{:?}", tokens);
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
        let mut tokens = VecDeque::from(lexer::lexer(r#"h1(){}p(){}"#.to_string()).unwrap());
        println!("{:?}", tokens);
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
