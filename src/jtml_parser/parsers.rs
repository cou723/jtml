use std::collections::VecDeque;

use crate::jtml_lexer::JtmlToken;

use self::ast::root::AstRoot;

use super::ParserError;

mod ast;
mod attributes;
mod document;
mod node;
mod nodes;
mod one_token;

pub fn parse(tokens: &mut VecDeque<JtmlToken>) -> Result<AstRoot, ParserError> {
    document::parse(tokens)
}

pub fn is_self_terminating_tag(tag_name: &String) -> bool {
    let empty_elements = vec![
        "br", "hr", "img", "input", "meta", "area", "base", "col", "embed", "keygen", "link",
        "param", "source",
    ];
    empty_elements.contains(&tag_name.as_str())
}

#[cfg(test)]
mod test {

    use test::ast::{node::Element, Node};

    use crate::jtml_lexer::test_utils::lexer;

    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn node_with_contents() {
        let mut tokens = lexer(r#"p(){"hello""world"}"#);
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            Node::Element(Element {
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
        let result = node::parse(&mut tokens);
        assert_eq!(
            result.unwrap(),
            Node::Element(Element {
                tag_name: "p".to_string(),
                attributes: VecDeque::from(vec![]),
                children: VecDeque::from(vec![Node::Element(Element {
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
        let result = document::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            AstRoot {
                elements: VecDeque::from(vec![
                    Node::Element(Element {
                        tag_name: "h1".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    }),
                    Node::Element(Element {
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
        let result = document::parse(&mut tokens);

        assert_eq!(
            result.unwrap(),
            AstRoot {
                elements: VecDeque::from(vec![
                    Node::Element(Element {
                        tag_name: "h1".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    }),
                    Node::Element(Element {
                        tag_name: "p".to_string(),
                        attributes: VecDeque::from(vec![]),
                        children: VecDeque::from(vec![])
                    })
                ])
            }
        );
    }
}
