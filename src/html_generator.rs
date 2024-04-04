use std::collections::VecDeque;

use crate::{
    html_generator_error::HtmlGeneratorError, jtml_lexer::lexer, jtml_parser::parser::jtml_parser,
};

pub(crate) fn html_generator(
    text: String,
    ignore_comment: bool,
) -> Result<String, HtmlGeneratorError> {
    let mut tokens = VecDeque::from(match lexer(text) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(HtmlGeneratorError::UnexpectedToken(e.to_string()));
        }
    });

    let ast = match jtml_parser(&mut tokens) {
        Ok(ast) => ast,
        Err(e) => return Err(HtmlGeneratorError::UnexpectedToken(e.to_string())),
    };
    Ok(ast.to_html(ignore_comment))
}
