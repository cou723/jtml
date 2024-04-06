use crate::jtml_parser::convert::Convert;

mod element;

pub use element::Element;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
}

impl Convert for Node {
    fn to_html(&self, ignore_comment: bool) -> String {
        match self {
            Node::Element(element) => element.to_html(ignore_comment),
            Node::Text(text) => text.to_string(),
            Node::Comment(text) => {
                if ignore_comment {
                    return "".to_string();
                }
                format!("<!--{}-->", text)
            }
        }
    }

    fn to_jtml(&self, ignore_comment: bool) -> String {
        match self {
            Node::Element(element) => element.to_jtml(ignore_comment),
            Node::Text(text) => format!("\"{}\"", text),
            Node::Comment(text) => {
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

    use super::Node;

    #[test]
    fn html_comment() {
        let comment = Node::Comment("".to_string());
        assert_eq!(comment.to_html(false), "<!---->");

        let comment = Node::Comment("comment".to_string());
        assert_eq!(comment.to_html(false), "<!--comment-->");
    }

    #[test]
    fn jtml_comment() {
        let comment = Node::Comment("".to_string());
        assert_eq!(comment.to_jtml(false), "// ");

        let comment = Node::Comment("comment".to_string());
        assert_eq!(comment.to_jtml(false), "// comment");
    }

    #[test]
    fn html_text() {
        let comment = Node::Text("".to_string());
        assert_eq!(comment.to_html(false), "");

        let comment = Node::Text("comment".to_string());
        assert_eq!(comment.to_html(false), "comment");
    }

    #[test]
    fn jtml_text() {
        let comment = Node::Text("".to_string());
        assert_eq!(comment.to_jtml(false), "\"\"");

        let comment = Node::Text("".to_string());
        assert_eq!(comment.to_jtml(false), "\"\"");
    }
}
