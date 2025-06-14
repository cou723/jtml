use std::collections::VecDeque;

use crate::{html_converter::Convert, jtml_parser::parsers::ast::Node};

pub type Children = VecDeque<Node>;

impl Convert for Children {
    fn to_html(&self, ignore_comment: bool) -> String {
        self.iter()
            .map(|element| element.to_html(ignore_comment))
            .collect::<String>()
    }

    fn to_jtml(&self, ignore_comment: bool, indent_depth: usize) -> String {
        if self.len() == 0 {
            "".to_string()
        } else {
            format!(
                "\n{}",
                self.iter()
                    .map(|element| element.to_jtml(ignore_comment, indent_depth + 1))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
    }
}
