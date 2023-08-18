use super::child_element::Child;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub elements: VecDeque<Child>,
}

impl Document {
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        for element in &self.elements {
            html.push_str(&element.to_html());
        }
        html
    }
}
