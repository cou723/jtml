use super::child_element::Child;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub elements: VecDeque<Child>,
}

impl Document {
    pub fn to_html(&self, ignore_comment: bool) -> String {
        let mut html = String::new();
        for element in &self.elements {
            html.push_str(&element.to_html(ignore_comment));
        }
        html
    }

    pub fn to_jtml(&self, ignore_comment: bool) -> String {
        let mut jtml = String::new();
        for element in &self.elements {
            jtml.push_str(&element.to_html(ignore_comment));
        }
        jtml
    }
}
