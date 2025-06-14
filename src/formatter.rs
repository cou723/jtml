use std::collections::VecDeque;

use crate::{html_converter::HtmlConverterError, jtml_lexer::lexer, jtml_parser};

pub fn format(text: String) -> Result<String, HtmlConverterError> {
    let mut tokens = VecDeque::from(match lexer(text) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(HtmlConverterError::LexerError(e));
        }
    });

    let ast = match jtml_parser::parse(&mut tokens) {
        Ok(ast) => ast,
        Err(e) => return Err(HtmlConverterError::ParseError(e)),
    };
    Ok(ast.to_jtml(false))
}

#[cfg(test)]
mod test {

    #[test]
    fn single_simple_element() {
        use super::*;
        let result = format("p(){}".to_string());
        assert_eq!(result.unwrap(), "p(){\n}".to_string());

        let result = format("img()".to_string());
        assert_eq!(result.unwrap(), "img()".to_string());

        let result = format("\"string literal\"".to_string());
        assert_eq!(result.unwrap(), "\"string literal\"".to_string());

        let result = format("// comment".to_string());
        assert_eq!(result.unwrap(), "// comment".to_string());
    }

    #[test]
    fn single_element_with_attribute() {
        use super::*;
        let result = format("p(class=\"btn\"){}".to_string()).unwrap();
        assert_eq!(result, "p(class=\"btn\"){\n}".to_string());

        let result = format("img(href=\"./images/img.png\")".to_string()).unwrap();
        assert_eq!(result, "img(href=\"./images/img.png\")".to_string());
    }

    #[test]
    fn single_element_with_child() {
        use super::*;
        let result = format(r#"p(){p(){"hello"}}"#.to_string()).unwrap();
        assert_eq!(
            result,
            r#"p(){
    p(){
        "hello"
    }
}"#
            .to_string()
        );
    }

    #[test]
    fn example_head() {
        use super::*;
        let result = format(
            r#"
head(){
    meta(charset="UTF-8")
    meta(http-equiv="X-UA-Compatible" content="IE=edge")
    title(){"document"}
}"#
            .to_string(),
        )
        .unwrap();
        assert_eq!(
            result,
            r#"head(){
    meta(charset="UTF-8")
    meta(http-equiv="X-UA-Compatible" content="IE=edge")
    title(){
        "document"
    }
}"#
        )
    }

    #[test]
    fn normal() {
        use super::*;
        let result = format(
            r#"
html(lang="ja"){
    head(){
        meta(charset="UTF-8")
        meta(http-equiv="X-UA-Compatible" content="IE=edge")
        meta(name="viewport" content="width=device-width" initial-scale="1.0")
        title(){"document"}
    }
    body(){
        main(){
            h1(){"Hello World!"}
            img(hoge="hoge" huga="huga")
        }
    }
}"#
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            result,
            r#"html(lang="ja"){
    head(){
        meta(charset="UTF-8")
        meta(http-equiv="X-UA-Compatible" content="IE=edge")
        meta(name="viewport" content="width=device-width" initial-scale="1.0")
        title(){
            "document"
        }
    }
    body(){
        main(){
            h1(){
                "Hello World!"
            }
            img(hoge="hoge" huga="huga")
        }
    }
}"#
            .to_string()
        )
    }
}
