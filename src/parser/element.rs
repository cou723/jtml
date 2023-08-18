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
    pub(crate) fn to_html(&self) -> String {
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
            html.push_str(&child.to_html());
        }
        html.push_str(&format!("</{}>", self.element_name));
        html
    }
}
