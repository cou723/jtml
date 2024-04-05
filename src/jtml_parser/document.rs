use super::ast_node::AstNode;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentNode {
    pub elements: VecDeque<AstNode>,
}

impl DocumentNode {
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
            jtml.push_str(&element.to_jtml(ignore_comment));
            jtml.push_str("\n");
        }
        jtml
    }
}
