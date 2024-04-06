use std::collections::VecDeque;

use crate::jtml_lexer::JtmlToken;

use self::nodes::document::DocumentNode;

use super::parser_error::ParserError;

mod ast_node_parser;
mod ast_nodes_parser;
mod attributes_parser;
mod document_parser;
mod nodes;
mod one_token_parser;

pub fn parse(tokens: &mut VecDeque<JtmlToken>) -> Result<DocumentNode, ParserError> {
    document_parser::parse(tokens)
}

#[cfg(test)]
mod test {
    use nodes::ast::Node;

    use crate::jtml_lexer::test_utils::lexer;
    use crate::jtml_parser::parsers::nodes::ast::element;

    use super::nodes;

    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn node_with_contents() {
        let mut tokens = lexer(r#"p(){"hello""world"}"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            Node::Element(element::Node {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![
                    Node::Text("hello".to_string()),
                    Node::Text("world".to_string())
                ])
            })
        );
    }

    #[test]
    fn node_with_child_elements() {
        let mut tokens = lexer(r#"p(){p(){"hello"}}"#);
        let result = ast_node_parser::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            Node::Element(element::Node {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![Node::Element(element::Node {
                    tag_name: "p".to_string(),
                    attributes: VecDeque::from(vec![]),
                    children: VecDeque::from(vec![Node::Text("hello".to_string())])
                })])
            })
        );
    }

    #[test]
    fn document() {
        let mut tokens = lexer(r#"h1(){}p(){}"#);
        let result = document_parser::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            DocumentNode {
                elements: VecDeque::from(vec![
                    Node::Element(element::Node {
                        tag_name: "h1".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    }),
                    Node::Element(element::Node {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    })
                ])
            }
        );
    }

    #[test]
    fn comment() {
        let mut tokens = lexer(r#"h1(){}p(){}"#);
        let result = document_parser::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            DocumentNode {
                elements: VecDeque::from(vec![
                    Node::Element(element::Node {
                        tag_name: "h1".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    }),
                    Node::Element(element::Node {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    })
                ])
            }
        );
    }
}
