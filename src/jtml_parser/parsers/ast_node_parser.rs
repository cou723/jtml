use super::ast_nodes_parser;
use super::attributes_parser;
use super::one_token_parser;

use super::super::parser_error::ParserError;

use crate::jtml_lexer::Kind;
use crate::jtml_parser::ast_node::AstNode;

use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::element::is_self_terminating_tag;
use crate::jtml_parser::element::ElementNode;

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

    one_token_parser::parse(JtmlToken::LeftParen, tokens)?;
    let attributes = attributes_parser::parse(tokens)?;
    one_token_parser::parse(JtmlToken::RightParen, tokens)?;

    if is_self_terminating_tag(&element_name) {
        return Ok(AstNode::Element(ElementNode {
            tag_name: element_name,
            attributes: attributes,
            children: VecDeque::from(vec![]),
        }));
    }
    one_token_parser::parse(JtmlToken::LeftBracket, tokens)?;
    let children = ast_nodes_parser::parse(tokens).0;
    one_token_parser::parse(JtmlToken::RightBracket, tokens)?;

    Ok(AstNode::Element(ElementNode {
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
    use crate::jtml_parser::parser_error::ParserError;
    use crate::jtml_parser::{ast_node::AstNode, element::ElementNode, parsers::ast_node_parser};

    #[test]
    fn element() {
        let mut tokens = lexer(r#"p(){}"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            AstNode::Element(ElementNode {
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
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            AstNode::Element(ElementNode {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![("width".to_string(), "100".to_string())]),
                children: VecDeque::from(vec![])
            })
        );
    }
    #[test]
    fn element_with_string() {
        let mut tokens = lexer(r#"p(){"hello"}"#);
        let result = ast_node_parser::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            AstNode::Element(ElementNode {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![AstNode::Text("hello".to_string())])
            })
        );
    }

    #[test]
    fn node_with_child_node() {
        let mut tokens = lexer(r#"p(){p(){"test"}p(){"test1""test2"}}}"#);
        let result = ast_node_parser::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            AstNode::Element(ElementNode {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![
                    AstNode::Element(ElementNode {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![AstNode::Text("test".to_string())])
                    }),
                    AstNode::Element(ElementNode {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![
                            AstNode::Text("test1".to_string()),
                            AstNode::Text("test2".to_string())
                        ])
                    })
                ])
            })
        );
    }

    #[test]
    fn invalid_element_right_bracket() {
        let mut tokens = lexer(r#"p(){"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap_err(),
            ParserError::TokenIsNotEnough(vec![Kind::RightBracket])
        );
    }

    #[test]
    fn invalid_element_left_bracket() {
        let mut tokens = lexer(r#"p()"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap_err(),
            ParserError::TokenIsNotEnough(vec![Kind::LeftBracket])
        );
    }

    #[test]
    fn invalid_element_right_paren() {
        let mut tokens = lexer(r#"p("#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap_err(),
            ParserError::TokenIsNotEnough(vec![Kind::RightParen])
        );
    }

    #[test]
    fn invalid_element_left_paren() {
        let mut tokens = lexer(r#"p)"#);
        let result = ast_node_parser::parse(&mut tokens);
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
