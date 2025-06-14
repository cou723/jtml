use crate::{html_converter::Convert, jtml_parser::parsers::is_self_terminating_tag};

mod attributes;
mod children;

use self::{attributes::Attributes, children::Children};

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

    fn to_jtml(&self, ignore_comment: bool, indent_depth: usize) -> String {
        // 子要素を持たない要素の場合
        if is_self_terminating_tag(&self.tag_name) {
            format!(
                "{}{}({})",
                "    ".repeat(indent_depth),
                self.tag_name,
                self.attributes.to_jtml(ignore_comment, indent_depth)
            )
        } else {
            format!(
                "{}{}({}){{{}\n{}}}",
                "    ".repeat(indent_depth),
                self.tag_name,
                self.attributes.to_jtml(ignore_comment, indent_depth),
                self.children.to_jtml(ignore_comment, indent_depth),
                "    ".repeat(indent_depth),
            )
        }
    }
}

// test
#[cfg(test)]
mod test {

    use crate::{
        html_converter::Convert,
        jtml_parser::parsers::ast::{node::Element, Node},
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
        assert_eq!(element.to_jtml(false, 0), "p(){\n}");

        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![Node::Text("test".to_string())]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(element.to_jtml(false, 0), "p(){\n    \"test\"\n}")
    }

    #[test]
    fn element_with_attribute() {
        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::from(vec![("class".to_string(), "btn".to_string())]),
            children: Children::new(),
        };
        assert_eq!(element.to_html(false), "<p class=\"btn\"></p>");
        assert_eq!(element.to_jtml(false, 0), "p(class=\"btn\"){\n}");

        let element = Element {
            tag_name: "img".to_string(),
            attributes: Attributes::from(vec![(
                "href".to_string(),
                "./images/img.png".to_string(),
            )]),
            children: Children::new(),
        };
        assert_eq!(element.to_html(false), "<img href=\"./images/img.png\"/>");
        assert_eq!(element.to_jtml(false, 0), "img(href=\"./images/img.png\")");
    }

    #[test]
    fn element_with_child() {
        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![Node::Text("test".to_string())]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(
            element.to_jtml(false, 0),
            r#"p(){
    "test"
}"#
        );

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
        assert_eq!(
            element.to_jtml(false, 0),
            r#"p(){
    p(){
    }
}"#
        );

        let element = Element {
            tag_name: "p".to_string(),
            attributes: Attributes::new(),
            children: Children::from(vec![
                Node::Text("te".to_string()),
                Node::Text("st".to_string()),
            ]),
        };
        assert_eq!(element.to_html(false), "<p>test</p>");
        assert_eq!(
            element.to_jtml(false, 0),
            r#"p(){
    "te"
    "st"
}"#
        );

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
        assert_eq!(
            element.to_jtml(false, 0),
            r#"p(){
    "test"
    p(){
    }
}"#
        );
    }
}
