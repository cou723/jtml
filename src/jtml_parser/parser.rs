use std::collections::VecDeque;

use crate::jtml_lexer::JtmlToken;

use super::{document, parser_error::ParserError};

mod ast_node_parser;
mod ast_nodes_parser;
mod attributes_parser;
mod document_parser;
mod one_token_parser;
pub fn jtml_parser(tokens: &mut VecDeque<JtmlToken>) -> Result<document::Document, ParserError> {
    document_parser::parser(tokens)
}

#[cfg(test)]
mod test {
    use test::document::Document;

    use super::*;
    use std::collections::VecDeque;

    use crate::{
        jtml_lexer::test_utils::lexer,
        jtml_parser::{ast_node::AstNode, element::Element},
    };

    #[test]
    fn test_node_with_contents() {
        let mut tokens = lexer(r#"p(){"hello""world"}"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            AstNode::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![
                    AstNode::Text("\"hello\"".to_string()),
                    AstNode::Text("\"world\"".to_string())
                ])
            })
        );
    }

    #[test]
    fn test_document() {
        let mut tokens = lexer(r#"h1(){}p(){}"#);
        let result = document_parser::parser(&mut tokens);

        assert_eq!(
            result.unwrap(),
            Document {
                elements: VecDeque::from(vec![
                    AstNode::Element(Element {
                        tag_name: "h1".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    }),
                    AstNode::Element(Element {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    })
                ])
            }
        );
    }

    #[test]
    fn test_comment() {
        let mut tokens = lexer(r#"h1(){}p(){}"#);
        let result = document_parser::parser(&mut tokens);

        assert_eq!(
            result.unwrap(),
            Document {
                elements: VecDeque::from(vec![
                    AstNode::Element(Element {
                        tag_name: "h1".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    }),
                    AstNode::Element(Element {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    })
                ])
            }
        );
    }
}
