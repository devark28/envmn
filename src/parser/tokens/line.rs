use crate::parser::tokens::token_name::TokenName;
use crate::parser::tokens::variable::Variable;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    Comment(String),
    Variable(Variable),
    Empty,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Line::Comment(comment) => write!(f, "# {comment}"),
            Line::Variable(variable) => write!(f, "{variable}"),
            Line::Empty => writeln!(f),
        }
    }
}

impl TokenName for Line {
    fn name() -> &'static str {
        "Line"
    }
}
