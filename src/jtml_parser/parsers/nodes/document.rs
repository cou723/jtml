use crate::jtml_parser::convert::Convert;

use super::ast::Node;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentNode {
    pub elements: VecDeque<Node>,
}

impl DocumentNode {
    pub fn to_html(&self, ignore_comment: bool) -> String {
        self.elements
            .iter()
            .map(|element| element.to_html(ignore_comment))
            .collect::<String>()
    }

    pub fn to_jtml(&self, ignore_comment: bool) -> String {
        self.elements
            .iter()
            .map(|element| element.to_jtml(ignore_comment))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
