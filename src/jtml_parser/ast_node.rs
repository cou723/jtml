use crate::jtml_parser::element::Element;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Element(Element),
    Text(String),
    Comment(String),
}

impl AstNode {
    pub(crate) fn to_html(&self, ignore_comment: bool) -> String {
        match self {
            AstNode::Element(element) => element.to_html(ignore_comment),
            AstNode::Text(text) => text.clone().trim_matches('"').to_string(),
            AstNode::Comment(text) => {
                if ignore_comment {
                    return "".to_string();
                }
                format!("<!--{}-->", text)
            }
        }
    }

    pub(crate) fn to_jtml(&self, ignore_comment: bool) -> String {
        match self {
            AstNode::Element(element) => element.to_jtml(ignore_comment),
            AstNode::Text(text) => text.clone().trim_matches('"').to_string(),
            AstNode::Comment(text) => {
                if ignore_comment {
                    return "".to_string();
                }
                format!("// {}", text)
            }
        }
    }
}

// test
#[cfg(test)]
mod test {

    use super::AstNode;

    #[test]
    fn element() {
        let comment = AstNode::Comment("".to_string());
        assert_eq!(comment.to_html(false), "<!---->");
        assert_eq!(comment.to_jtml(false), "// ");

        let comment = AstNode::Comment("comment".to_string());
        assert_eq!(comment.to_html(false), "<!--comment-->");
        assert_eq!(comment.to_jtml(false), "// comment");
    }
}
