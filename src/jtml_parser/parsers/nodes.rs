use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::ParserError;

use std::collections::VecDeque;

use super::ast::Node;
use super::node;

pub(crate) fn parse(tokens: &mut VecDeque<JtmlToken>) -> (VecDeque<Node>, ParserError) {
    let mut elements: VecDeque<Node> = VecDeque::new();
    loop {
        match node::parse(tokens) {
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
            parser_errors::ParserError,
            parsers::{
                ast::{node::Element, Node},
                nodes,
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
        let parsed = nodes::parse(&mut tokens).0;
        assert_eq!(
            parsed,
            VecDeque::from(vec![
                Node::Element(Element {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![]),
                    children: VecDeque::from(vec![])
                }),
                Node::Element(Element {
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
        let parsed = nodes::parse(&mut tokens).0;
        assert_eq!(
            parsed,
            VecDeque::from(vec![
                Node::Text("stringliteral".to_string()),
                Node::Element(Element {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![("a".to_string(), "b".to_string())]),
                    children: VecDeque::from(vec![Node::Text("child".to_string())])
                }),
                Node::Comment("comment".to_string()),
                Node::Element(Element {
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
        let parsed = nodes::parse(&mut tokens);
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
        let parsed = nodes::parse(&mut tokens);
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
