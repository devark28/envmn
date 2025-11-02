use crate::parser::tokens::variable::Variable;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, Hash)]
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

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (Line::Variable(var), Line::Variable(other_var)) => var == other_var,
            _ => false,
        }
    }
}
