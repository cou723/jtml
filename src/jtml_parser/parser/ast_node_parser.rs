use super::ast_nodes_parser;
use super::attributes_parser;
use super::one_token_parser;

use super::super::parser_error::ParserError;

use crate::jtml_parser::ast_node::AstNode;

use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::element::Element;

use std::collections::VecDeque;

pub(crate) fn parse(tokens: &mut VecDeque<JtmlToken>) -> Result<AstNode, ParserError> {
    // elementの場合はelement_nameを取得
    // StringLiteral, Commentの場合はそのまま返す
    let element_name = match tokens.front() {
        Some(token) => match token {
            JtmlToken::StringLiteral(text) => {
                let new_text = text.clone();
                tokens.pop_front();
                return Ok(AstNode::Text(new_text));
            }
            JtmlToken::Comment(text) => {
                let new_text = text.clone();
                tokens.pop_front();
                return Ok(AstNode::Comment(new_text));
            }
            JtmlToken::Identifier(id) => id.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    JtmlToken::Identifier("element-name".to_string()),
                    token.to_string(),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(JtmlToken::Identifier(
                "element-name".to_string(),
            )))
        }
    };
    tokens.pop_front();

    one_token_parser::parse(JtmlToken::LeftParen, tokens)?;
    let attributes = attributes_parser::parse(tokens)?;
    one_token_parser::parse(JtmlToken::RightParen, tokens)?;

    one_token_parser::parse(JtmlToken::LeftBracket, tokens)?;
    let children = ast_nodes_parser::parse(tokens)?;
    one_token_parser::parse(JtmlToken::RightBracket, tokens)?;

    Ok(AstNode::Element(Element {
        tag_name: element_name,
        attributes: attributes,
        children: children,
    }))
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::jtml_lexer::test_utils::lexer;
    use crate::jtml_parser::{ast_node::AstNode, element::Element, parser::ast_node_parser};

    #[test]
    fn test_element() {
        let mut tokens = lexer(r#"p(){}"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            AstNode::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![])
            })
        );
    }

    #[test]
    fn test_element_with_attribute() {
        let mut tokens = lexer(r#"p(width="100"){}"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            AstNode::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![("width".to_string(), r#""100""#.to_string())]),
                children: VecDeque::from(vec![])
            })
        );
    }
    #[test]
    fn test_element_with_string() {
        let mut tokens = lexer(r#"p(){"hello"}"#);
        let result = ast_node_parser::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            AstNode::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![AstNode::Text("\"hello\"".to_string())])
            })
        );
    }

    #[test]
    fn test_node_with_child_node() {
        let mut tokens = lexer(r#"p(){p(){"test"}p(){"test1""test2"}}}"#);
        let result = ast_node_parser::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            AstNode::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![
                    AstNode::Element(Element {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![AstNode::Text("\"test\"".to_string())])
                    }),
                    AstNode::Element(Element {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![
                            AstNode::Text("\"test1\"".to_string()),
                            AstNode::Text("\"test2\"".to_string())
                        ])
                    })
                ])
            })
        );
    }
}
