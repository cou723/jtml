use super::{ast_node::AstNode, convert::Convert};

use std::collections::VecDeque;

pub type Attribute = (String, String);

impl Convert for Attribute {
    fn to_html(&self, _: bool) -> String {
        return format!("{}=\"{}\"", self.0, self.1);
    }

    fn to_jtml(&self, _: bool) -> String {
        return format!("{}=\"{}\"", self.0, self.1);
    }
}

pub type Attributes = VecDeque<Attribute>;

impl Convert for Attributes {
    fn to_html(&self, ignore_comment: bool) -> String {
        let mut html: Vec<String> = Vec::new();
        for attribute in self {
            html.push(attribute.to_html(ignore_comment));
        }
        return html.join(" ");
    }

    fn to_jtml(&self, ignore_comment: bool) -> String {
        let mut jtml: Vec<String> = Vec::new();
        for attribute in self {
            jtml.push(attribute.to_html(ignore_comment));
        }
        return jtml.join(" ");
    }
}

pub type Children = VecDeque<AstNode>;
#[derive(Debug, Clone, PartialEq)]
pub struct ElementNode {
    pub tag_name: String,
    pub attributes: Attributes,
    pub children: Children,
}

pub fn is_self_terminating_tag(tag_name: &String) -> bool {
    let empty_elements = vec![
        "br", "hr", "img", "input", "meta", "area", "base", "col", "embed", "keygen", "link",
        "param", "source",
    ];
    empty_elements.contains(&tag_name.as_str())
}

impl Convert for ElementNode {
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

    use super::{AstNode, Attributes, Children, ElementNode};
    use crate::jtml_parser::element::Convert;

    #[test]
    fn element() {
        let element = ElementNode {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::new(),
        };
        assert_eq!(element.to_html(false), "<p></p>");
        assert_eq!(element.to_jtml(false), "p(){}");

        let element = ElementNode {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![AstNode::Text("test".to_string())]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(element.to_jtml(false), "p(){\"test\"}")
    }

    #[test]
    fn element_with_attribute() {
        let element = ElementNode {
            tag_name: "p".to_string(),
            attributes: Attributes::from(vec![("class".to_string(), "btn".to_string())]),
            children: Children::new(),
        };
        assert_eq!(element.to_html(false), "<p class=\"btn\"></p>");
        assert_eq!(element.to_jtml(false), "p(class=\"btn\"){}");

        let element = ElementNode {
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
        let element = ElementNode {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![AstNode::Text("test".to_string())]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(element.to_jtml(false), r#"p(){"test"}"#);

        let element = ElementNode {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![AstNode::Element(ElementNode {
                tag_name: "p".to_string(),
                attributes: Attributes::new(),
                children: Children::new(),
            })]),
        };
        assert_eq!(element.to_html(false), "<p><p></p></p>");
        assert_eq!(element.to_jtml(false), "p(){p(){}}");

        let element = ElementNode {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                AstNode::Text("test".to_string()),
                AstNode::Text("test".to_string()),
            ]),
        };
        assert_eq!(element.to_html(false), "<p>testtest</p>");
        assert_eq!(element.to_jtml(false), r#"p(){"test""test"}"#);

        let element = ElementNode {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                AstNode::Text("test".to_string()),
                AstNode::Element(ElementNode {
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
