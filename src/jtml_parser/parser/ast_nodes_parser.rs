use crate::jtml_parser::ast_node::AstNode;

use crate::jtml_lexer::JtmlToken;
use crate::jtml_parser::parser_error::ParserError;

use std::collections::VecDeque;

use super::ast_node_parser;

pub(crate) fn parse(tokens: &mut VecDeque<JtmlToken>) -> (VecDeque<AstNode>, ParserError) {
    let mut elements: VecDeque<AstNode> = VecDeque::new();
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
            ast_node::AstNode, element::ElementNode, parser::ast_nodes_parser,
            parser_error::ParserError,
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
                AstNode::Element(ElementNode {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![]),
                    children: VecDeque::from(vec![])
                }),
                AstNode::Element(ElementNode {
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
                AstNode::Text("\"stringliteral\"".to_string()),
                AstNode::Element(ElementNode {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![("a".to_string(), "\"b\"".to_string())]),
                    children: VecDeque::from(vec![AstNode::Text("\"child\"".to_string())])
                }),
                AstNode::Comment("comment".to_string()),
                AstNode::Element(ElementNode {
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
