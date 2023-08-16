pub(crate) enum ConvertError {
    UnexpectedToken(String),
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConvertError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
        }
    }
}
