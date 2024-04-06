use std::collections::VecDeque;

use crate::{jtml_lexer::JtmlToken, jtml_parser::ParserError};

pub(crate) fn parse(
    expect: JtmlToken,
    tokens: &mut VecDeque<JtmlToken>,
) -> Result<(), ParserError> {
    match tokens.remove(0) {
        Some(token) => {
            if token == expect {
                Ok(())
            } else {
                Err(ParserError::UnexpectedToken(
                    expect.into(),
                    token,
                    Some(tokens.clone()),
                ))
            }
        }
        None => Err(ParserError::TokenIsNotEnough(vec![expect.into()])),
    }
}

#[cfg(test)]
mod test {
    use crate::jtml_lexer::Kind;

    #[test]
    fn normal() {
        use super::*;
        use crate::jtml_lexer::test_utils::lexer;
        let mut tokens = lexer(r#"("#);
        let result = parse(JtmlToken::LeftParen, &mut tokens);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn error() {
        use super::*;
        use crate::jtml_lexer::test_utils::lexer;
        let mut tokens = lexer(r#"("#);
        let result = parse(JtmlToken::RightParen, &mut tokens);
        assert_eq!(
            result,
            Err(ParserError::UnexpectedToken(
                Kind::RightParen,
                JtmlToken::LeftParen,
                Some(tokens.clone())
            ))
        );
    }
}
