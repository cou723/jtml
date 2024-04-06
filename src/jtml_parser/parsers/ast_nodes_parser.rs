use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::parser_error::ParserError;

use std::collections::VecDeque;

use super::ast_node_parser;
use super::nodes::ast::Node;

pub(crate) fn parse(tokens: &mut VecDeque<JtmlToken>) -> (VecDeque<Node>, ParserError) {
    let mut elements: VecDeque<Node> = VecDeque::new();
    loop {
        match ast_node_parser::parse(tokens) {
            Ok(e) => {
                elements.push_back(e);
            }
            Err(e) => return (elements, e),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::{
        jtml_lexer::{test_utils::lexer, Kind},
        jtml_parser::{
            parser_error::ParserError,
            parsers::{
                ast_nodes_parser,
                nodes::ast::{self, element::Node},
            },
        },
    };

    #[test]
    fn normal() {
        let mut tokens = lexer(
            r#"
        p(){

        }
        p(){

        }
        "#,
        );
        let parsed = ast_nodes_parser::parse(&mut tokens).0;
        assert_eq!(
            parsed,
            VecDeque::from(vec![
                ast::Node::Element(Node {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![]),
                    children: VecDeque::from(vec![])
                }),
                ast::Node::Element(Node {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![]),
                    children: VecDeque::from(vec![])
                })
            ])
        );
    }

    #[test]
    fn complicated() {
        let mut tokens = lexer(
            r#"
        "stringliteral"
        p(a="b"){
            "child"
        }
        // comment
        p(){

        }
        "#,
        );
        let parsed = ast_nodes_parser::parse(&mut tokens).0;
        assert_eq!(
            parsed,
            VecDeque::from(vec![
                ast::Node::Text("stringliteral".to_string()),
                ast::Node::Element(Node {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![("a".to_string(), "b".to_string())]),
                    children: VecDeque::from(vec![ast::Node::Text("child".to_string())])
                }),
                ast::Node::Comment("comment".to_string()),
                ast::Node::Element(Node {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![]),
                    children: VecDeque::from(vec![])
                })
            ])
        );
    }

    #[test]
    fn error_handling_check() {
        let mut tokens = lexer(
            r#"
        p(a="b"){
            "child"
        }
        // comment
        p(){
        "#,
        );
        let parsed = ast_nodes_parser::parse(&mut tokens);
        assert_eq!(
            parsed.1,
            (ParserError::TokenIsNotEnough(vec![Kind::RightBracket]))
        );

        let mut tokens = lexer(
            r#"
        p(a="b"){
            "child"
        }
        // comment
        p(){(
        "#,
        );
        let parsed = ast_nodes_parser::parse(&mut tokens);
        assert_eq!(
            parsed.1,
            (ParserError::UnexpectedToken(
                Kind::RightBracket,
                crate::jtml_lexer::JtmlToken::LeftParen,
                None
            ))
        )
    }
}
