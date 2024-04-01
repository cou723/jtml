use crate::parser::element::Element;

#[derive(Debug, Clone, PartialEq)]
pub enum Child {
    Element(Element),
    Text(String),
    Comment(String),
}

impl Child {
    pub(crate) fn to_html(&self, ignore_comment: bool) -> String {
        match self {
            Child::Element(element) => element.to_html(ignore_comment),
            Child::Text(text) => text.clone().trim_matches('"').to_string(),
            Child::Comment(text) => {
                if ignore_comment {
                    return "".to_string();
                }
                format!("<!--{}-->", text)
            }
        }
    }

    pub(crate) fn to_jtml(&self, ignore_comment: bool) -> String {
        match self {
            Child::Element(element) => element.to_jtml(ignore_comment),
            Child::Text(text) => text.clone().trim_matches('"').to_string(),
            Child::Comment(text) => {
                if ignore_comment {
                    return "".to_string();
                }
                format!("<!--{}-->", text)
            }
        }
    }
}
