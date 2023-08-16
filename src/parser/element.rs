use super::child_element::Child;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Element {
    pub(crate) element_name: String,
    pub(crate) attributes: VecDeque<(String, String)>,
    pub(crate) children: VecDeque<Child>,
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
