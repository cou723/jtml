use super::child_element::Child;

use std::collections::VecDeque;

pub type Attribute = (String, String);
pub type Attributes = VecDeque<Attribute>;
pub type Children = VecDeque<Child>;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub element_name: String,
    pub attributes: Attributes,
    pub children: Children,
}

impl Element {
    pub(crate) fn to_html(&self, ignore_comment: bool) -> String {
        let empty_elements = vec![
            "br", "hr", "img", "input", "meta", "area", "base", "col", "embed", "keygen", "link",
            "param", "source",
        ];
        let mut html = String::new();
        html.push_str(&format!("<{}", self.element_name));
        if empty_elements.contains(&self.element_name.as_str()) {
            for (key, value) in &self.attributes {
                html.push_str(&format!(" {}={}", key, value));
            }
            html.push_str(">");
            return html;
        }
        for (key, value) in &self.attributes {
            html.push_str(&format!(" {}={}", key, value));
        }
        html.push_str(">");
        for child in &self.children {
            html.push_str(&child.to_html(ignore_comment));
        }
        html.push_str(&format!("</{}>", self.element_name));
        html
    }

    pub(crate) fn to_jtml(&self, ignore_comment: bool) -> String {
        let empty_elements = vec![
            "br", "hr", "img", "input", "meta", "area", "base", "col", "embed", "keygen", "link",
            "param", "source",
        ];

        let mut jtml = String::new();

        jtml.push_str(&format!("{}(", self.element_name));

        // 子要素を持たない要素の場合
        if empty_elements.contains(&self.element_name.as_str()) {
            for (key, value) in &self.attributes {
                jtml.push_str(&format!(" {}={}", key, value));
            }
            jtml.push_str(")");
            return jtml;
        }

        // 子要素を持つ要素の場合
        for (key, value) in &self.attributes {
            jtml.push_str(&format!(" {}={}", key, value));
        }
        jtml.push_str("){");
        for child in &self.children {
            jtml.push_str(&child.to_jtml(ignore_comment));
        }
        jtml.push_str("}");
        jtml
    }
}
