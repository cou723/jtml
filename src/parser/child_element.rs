use crate::parser::element::Element;

#[derive(Debug, Clone, PartialEq)]
pub enum Child {
    Element(Element),
    Text(String),
}

impl Child {
    pub(crate) fn to_html(&self) -> String {
        match self {
            Child::Element(element) => element.to_html(),
            Child::Text(text) => text.clone().trim_matches('"').to_string(),
        }
    }
}
