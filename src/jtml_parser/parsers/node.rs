use crate::jtml_lexer::Kind;

use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::parsers::ast::node::Element;
use crate::jtml_parser::parsers::ast::Node;
use crate::jtml_parser::parsers::attributes;
use crate::jtml_parser::parsers::is_self_terminating_tag;
use crate::jtml_parser::parsers::one_token;
use crate::jtml_parser::ParserError;

use std::collections::VecDeque;

use super::nodes;

pub(crate) fn parse(tokens: &mut VecDeque<JtmlToken>) -> Result<Node, ParserError> {
    // elementの場合はelement_nameを取得
    // StringLiteral, Commentの場合はそのまま返す
    let element_name = match tokens.front() {
        Some(token) => match token {
            JtmlToken::StringLiteral(text) => {
                let new_text = text.clone();
                tokens.pop_front();
                return Ok(Node::Text(new_text));
            }
            JtmlToken::Comment(text) => {
                let new_text = text.clone();
                tokens.pop_front();
                return Ok(Node::Comment(new_text));
            }
            JtmlToken::Identifier(id) => id.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken(
                    Kind::Identifier,
                    token.clone(),
                    Some(tokens.clone()),
                ))
            }
        },
        None => {
            return Err(ParserError::TokenIsNotEnough(vec![
                Kind::StringLiteral,
                Kind::Comment,
                Kind::Identifier,
            ]))
        }
    };
    tokens.pop_front();

    one_token::parse(JtmlToken::LeftParen, tokens)?;
    let attributes = attributes::parse(tokens)?;
    one_token::parse(JtmlToken::RightParen, tokens)?;

    if is_self_terminating_tag(&element_name) {
        return Ok(Node::Element(Element {
            tag_name: element_name,
            attributes: attributes,
            children: VecDeque::from(vec![]),
        }));
    }
    one_token::parse(JtmlToken::LeftBracket, tokens)?;
    let children = nodes::parse(tokens).0;
    one_token::parse(JtmlToken::RightBracket, tokens)?;

    Ok(Node::Element(Element {
        tag_name: element_name,
        attributes: attributes,
        children: children,
    }))
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::jtml_lexer::test_utils::lexer;
    use crate::jtml_lexer::Kind;
    use crate::jtml_parser::errors::ParserError;
    use crate::jtml_parser::parsers::ast::node::{Element, Node};
    use crate::jtml_parser::parsers::node;

    #[test]
    fn element() {
        let mut tokens = lexer(r#"p(){}"#);
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            Node::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![])
            })
        );
    }

    // 未実装
    // #[test]
    // fn element_non_child() {
    //     let mut tokens = lexer(r#"img()"#);
    //     let result = ast_node_parser::parse(&mut tokens);
    //     assert_eq!(
    //         result.unwrap(),
    //         AstNode::Element(ElementNode {
    //             tag_name: "img".to_string(),
    //             attributes: VecDeque::from(vec![]),
    //             children: VecDeque::from(vec![])
    //         })
    //     );
    // }

    #[test]
    fn element_with_attribute() {
        let mut tokens = lexer(r#"p(width="100"){}"#);
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            Node::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![("width".to_string(), "100".to_string())]),
                children: VecDeque::from(vec![])
            })
        );
    }
    #[test]
    fn element_with_string() {
        let mut tokens = lexer(r#"p(){"hello"}"#);
        let result = node::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            Node::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![Node::Text("hello".to_string())])
            })
        );
    }

    #[test]
    fn node_with_child_node() {
        let mut tokens = lexer(r#"p(){p(){"test"}p(){"test1""test2"}}}"#);
        let result = node::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            Node::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![
                    Node::Element(Element {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![Node::Text("test".to_string())])
                    }),
                    Node::Element(Element {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![
                            Node::Text("test1".to_string()),
                            Node::Text("test2".to_string())
                        ])
                    })
                ])
            })
        );
    }

    #[test]
    fn invalid_element_right_bracket() {
        let mut tokens = lexer(r#"p(){"#);
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap_err(),
            ParserError::TokenIsNotEnough(vec![Kind::RightBracket])
        );
    }

    #[test]
    fn invalid_element_left_bracket() {
        let mut tokens = lexer(r#"p()"#);
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap_err(),
            ParserError::TokenIsNotEnough(vec![Kind::LeftBracket])
        );
    }

    #[test]
    fn invalid_element_right_paren() {
        let mut tokens = lexer(r#"p("#);
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap_err(),
            ParserError::TokenIsNotEnough(vec![Kind::RightParen])
        );
    }

    #[test]
    fn invalid_element_left_paren() {
        let mut tokens = lexer(r#"p)"#);
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap_err(),
            ParserError::UnexpectedToken(
                Kind::LeftParen,
                crate::jtml_lexer::JtmlToken::RightParen,
                None
            )
        );
    }
}
