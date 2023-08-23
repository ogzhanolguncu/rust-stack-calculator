#[derive(Debug, PartialEq)]
pub enum ParseCustomError {
    EmptyExpression,
    NotEnoughArguments,
    UnknownOperator(char),
}

impl std::fmt::Display for ParseCustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmptyExpression => write!(f, "The expression is empty."),
            Self::NotEnoughArguments => write!(f, "The expression does not have enough arguments."),
            Self::UnknownOperator(op) => write!(f, "Unknown operator: {}", op),
        }
    }
}

impl std::error::Error for ParseCustomError {}
