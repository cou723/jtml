use std::collections::VecDeque;

use crate::html_converter::Convert;

use super::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct AstRoot {
    pub elements: VecDeque<Node>,
}

impl AstRoot {
    pub fn to_html(&self, ignore_comment: bool) -> String {
        self.elements
            .iter()
            .map(|element| element.to_html(ignore_comment))
            .collect::<String>()
    }

    pub fn to_jtml(&self, ignore_comment: bool) -> String {
        self.elements
            .iter()
            .map(|element| element.to_jtml(ignore_comment, 0))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
