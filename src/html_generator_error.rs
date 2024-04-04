use std::fmt::{Display, Formatter, Result};

pub(crate) enum HtmlGeneratorError {
    UnexpectedToken(String),
}

impl Display for HtmlGeneratorError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            HtmlGeneratorError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
        }
    }
}
