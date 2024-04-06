use std::collections::VecDeque;

use crate::{html_generator_error::HtmlGeneratorError, jtml_lexer::lexer, jtml_parser};

pub fn format(text: String) -> Result<String, HtmlGeneratorError> {
    let mut tokens = VecDeque::from(match lexer(text) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(HtmlGeneratorError::LexerError(e));
        }
    });

    let ast = match jtml_parser::parsers::parse(&mut tokens) {
        Ok(ast) => ast,
        Err(e) => return Err(HtmlGeneratorError::ParseError(e)),
    };
    Ok(ast.to_jtml(false))
}
