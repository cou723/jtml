use std::collections::VecDeque;

use crate::{
    html_generator_error::HtmlGeneratorError, jtml_lexer::lexer, jtml_parser::parsers::parse,
};

pub fn convert(jtml: String, ignore_comment: bool) -> Result<String, HtmlGeneratorError> {
    let mut tokens = VecDeque::from(match lexer(jtml) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(HtmlGeneratorError::LexerError(e));
        }
    });

    let ast = match parse(&mut tokens) {
        Ok(ast) => ast,
        Err(e) => return Err(HtmlGeneratorError::ParseError(e)),
    };
    Ok(ast.to_html(ignore_comment))
}

#[cfg(test)]
mod test {
    use super::convert;

    #[test]
    fn single_simple_element() {
        use super::*;
        let result = convert("p(){}".to_string(), false);
        assert_eq!(result.unwrap(), "<p></p>".to_string());

        let result = convert("img()".to_string(), false);
        assert_eq!(result.unwrap(), "<img/>".to_string());

        let result = convert("\"string literal\"".to_string(), false);
        assert_eq!(result.unwrap(), "string literal".to_string());

        let result = convert("// comment".to_string(), false);
        assert_eq!(result.unwrap(), "<!--comment-->".to_string());
    }

    #[test]
    fn single_element_with_attribute() {
        use super::*;
        let result = convert("p(class=\"btn\"){}".to_string(), false).unwrap();
        assert_eq!(result, "<p class=\"btn\"></p>".to_string());

        let result = convert("img(href=\"./images/img.png\")".to_string(), false).unwrap();
        assert_eq!(result, "<img href=\"./images/img.png\"/>".to_string());
    }

    #[test]
    fn single_element_with_child() {
        use super::*;
        let result = convert(r#"p(){p(){"hello"}}"#.to_string(), false).unwrap();
        assert_eq!(result, "<p><p>hello</p></p>".to_string());
    }

    #[test]
    fn example_head() {
        let result = convert(
            r#"
head(){
    meta(charset="UTF-8")
    meta(http-equiv="X-UA-Compatible" content="IE=edge")
    title(){"document"}
}"#
            .to_string(),
            false,
        )
        .unwrap();
        assert_eq!(
            result,
            r#"<head><meta charset="UTF-8"/><meta http-equiv="X-UA-Compatible" content="IE=edge"/><title>document</title></head>"#
        )
    }

    #[test]
    fn normal() {
        let result = convert(
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
            false,
        )
        .unwrap();

        assert_eq!(
            result,
            r#"<html lang="ja"><head><meta charset="UTF-8"/><meta http-equiv="X-UA-Compatible" content="IE=edge"/><meta name="viewport" content="width=device-width" initial-scale="1.0"/><title>document</title></head><body><main><h1>Hello World!</h1><img hoge="hoge" huga="huga"/></main></body></html>"#
                .replace("\n", "")
                .to_string()
        )
    }
}
