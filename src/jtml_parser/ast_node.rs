use crate::jtml_parser::element::ElementNode;

use super::convert::Convert;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Element(ElementNode),
    Text(String),
    Comment(String),
}

impl Convert for AstNode {
    fn to_html(&self, ignore_comment: bool) -> String {
        match self {
            AstNode::Element(element) => element.to_html(ignore_comment),
            AstNode::Text(text) => text.to_string(),
            AstNode::Comment(text) => {
                if ignore_comment {
                    return "".to_string();
                }
                format!("<!--{}-->", text)
            }
        }
    }

    fn to_jtml(&self, ignore_comment: bool) -> String {
        match self {
            AstNode::Element(element) => element.to_jtml(ignore_comment),
            AstNode::Text(text) => format!("\"{}\"", text),
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
    // Elementはより下位のモジュールでテストしているため、ここでは使用しない
    //

    use crate::jtml_parser::convert::Convert;

    use super::AstNode;

    #[test]
    fn html_comment() {
        let comment = AstNode::Comment("".to_string());
        assert_eq!(comment.to_html(false), "<!---->");

        let comment = AstNode::Comment("comment".to_string());
        assert_eq!(comment.to_html(false), "<!--comment-->");
    }

    #[test]
    fn jtml_comment() {
        let comment = AstNode::Comment("".to_string());
        assert_eq!(comment.to_jtml(false), "// ");

        let comment = AstNode::Comment("comment".to_string());
        assert_eq!(comment.to_jtml(false), "// comment");
    }

    #[test]
    fn html_text() {
        let comment = AstNode::Text("".to_string());
        assert_eq!(comment.to_html(false), "");

        let comment = AstNode::Text("comment".to_string());
        assert_eq!(comment.to_html(false), "comment");
    }

    #[test]
    fn jtml_text() {
        let comment = AstNode::Text("".to_string());
        assert_eq!(comment.to_jtml(false), "\"\"");

        let comment = AstNode::Text("".to_string());
        assert_eq!(comment.to_jtml(false), "\"\"");
    }
}
