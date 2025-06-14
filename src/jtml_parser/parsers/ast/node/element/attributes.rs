use std::collections::VecDeque;

use crate::html_converter::Convert;

pub type Attribute = (String, String);

impl Convert for Attribute {
    fn to_html(&self, _: bool) -> String {
        return format!("{}=\"{}\"", self.0, self.1);
    }

    fn to_jtml(&self, _: bool, _indent_depth: usize) -> String {
        return format!("{}=\"{}\"", self.0, self.1);
    }
}

pub type Attributes = VecDeque<Attribute>;

impl Convert for Attributes {
    fn to_html(&self, ignore_comment: bool) -> String {
        let mut html: Vec<String> = Vec::new();
        for attribute in self {
            html.push(attribute.to_html(ignore_comment));
        }
        return html.join(" ");
    }

    fn to_jtml(&self, ignore_comment: bool, indent_depth: usize) -> String {
        let mut jtml: Vec<String> = Vec::new();
        for attribute in self {
            jtml.push(attribute.to_jtml(ignore_comment, indent_depth));
        }
        return jtml.join(" ");
    }
}
