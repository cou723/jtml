use std::collections::VecDeque;

use crate::{convert_error::ConvertError, lexer::lexer, parser::parser};

pub(crate) fn convert(text: String) -> Result<String, ConvertError> {
    let mut tokens = VecDeque::from(match lexer(text) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(ConvertError::UnexpectedToken(e.to_string()));
        }
    });

    let ast = match parser(&mut tokens) {
        Ok(ast) => ast,
        Err(e) => return Err(ConvertError::UnexpectedToken(e.to_string())),
    };
    Ok(ast.to_html())
}
