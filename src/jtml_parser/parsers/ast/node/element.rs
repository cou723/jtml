use crate::jtml_parser::{convert::Convert, parsers::is_self_terminating_tag};

use std::collections::VecDeque;

mod attributes;

use self::attributes::Attributes;

use super::Node;

pub type Children = VecDeque<Node>;
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Attributes,
    pub children: Children,
}

impl Convert for Element {
    fn to_html(&self, ignore_comment: bool) -> String {
        let attributes = match self.attributes.to_html(ignore_comment).as_str() {
            "" => "".to_string(),
            s => format!(" {}", s),
        };

        if is_self_terminating_tag(&self.tag_name) {
            format!("<{}{}/>", self.tag_name, attributes)
        } else {
            format!(
                "<{}{}>{}</{}>",
                self.tag_name,
                attributes,
                self.children
                    .iter()
                    .map(|child| (child.to_html(ignore_comment).clone()))
                    .collect::<Vec<String>>()
                    .concat(),
                self.tag_name,
            )
        }
    }

    fn to_jtml(&self, ignore_comment: bool) -> String {
        // 子要素を持たない要素の場合
        if is_self_terminating_tag(&self.tag_name) {
            format!(
                "{}({})",
                self.tag_name,
                self.attributes.to_jtml(ignore_comment)
            )
        } else {
            format!(
                "{}({}){{{}}}",
                self.tag_name,
                self.attributes.to_jtml(ignore_comment),
                self.children
                    .iter()
                    .map(|child| (child.to_jtml(ignore_comment).clone()))
                    .collect::<Vec<String>>()
                    .concat(),
            )
        }
    }
}

// test
#[cfg(test)]
mod test {

    use crate::jtml_parser::{
        convert::Convert,
        parsers::ast::{node::Element, Node},
    };

    use super::{Attributes, Children};

    #[test]
    fn element() {
        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::new(),
        };
        assert_eq!(element.to_html(false), "<p></p>");
        assert_eq!(element.to_jtml(false), "p(){}");

        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![Node::Text("test".to_string())]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(element.to_jtml(false), "p(){\"test\"}")
    }

    #[test]
    fn element_with_attribute() {
        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::from(vec![("class".to_string(), "btn".to_string())]),
            children: Children::new(),
        };
        assert_eq!(element.to_html(false), "<p class=\"btn\"></p>");
        assert_eq!(element.to_jtml(false), "p(class=\"btn\"){}");

        let element = Element {
            tag_name: "img".to_string(),
            attributes: Attributes::from(vec![(
                "href".to_string(),
                "./images/img.png".to_string(),
            )]),
            children: Children::new(),
        };
        assert_eq!(element.to_html(false), "<img href=\"./images/img.png\"/>");
        assert_eq!(element.to_jtml(false), "img(href=\"./images/img.png\")");
    }

    #[test]
    fn element_with_child() {
        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![Node::Text("test".to_string())]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(element.to_jtml(false), r#"p(){"test"}"#);

        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![Node::Element(Element {
                tag_name: "p".to_string(),
                attributes: Attributes::new(),
                children: Children::new(),
            })]),
        };
        assert_eq!(element.to_html(false), "<p><p></p></p>");
        assert_eq!(element.to_jtml(false), "p(){p(){}}");

        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                Node::Text("te".to_string()),
                Node::Text("st".to_string()),
            ]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(element.to_jtml(false), r#"p(){"te""st"}"#);

        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                Node::Text("test".to_string()),
                Node::Element(Element {
                    tag_name: "p".to_string(),
                    attributes: Attributes::new(),
                    children: Children::new(),
                }),
            ]),
        };
        assert_eq!(element.to_html(false), "<p>test<p></p></p>");
        assert_eq!(element.to_jtml(false), "p(){\"test\"p(){}}");
    }
}
