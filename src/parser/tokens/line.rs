use crate::parser::tokens::variable::Variable;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash)]
pub enum Line {
    Comment(String),
    Variable(Variable),
}

impl Line {
    pub fn is_comment(&self) -> bool {
        matches!(self, Line::Comment(_))
    }
    pub fn is_variable(&self) -> bool {
        matches!(self, Line::Variable(_))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Line::Comment(comment) => write!(f, "# {comment}"),
            Line::Variable(variable) => write!(f, "{variable}"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_equality_by_variable() {
        let v1 = Variable::new("KEY", "value1");
        let v2 = Variable::new("KEY", "value2");
        let line1 = Line::Variable(v1);
        let line2 = Line::Variable(v2);
        assert_eq!(line1, line2);
    }

    #[test]
    fn test_line_inequality_by_comment() {
        let line1 = Line::Comment("comment".to_string());
        let line2 = Line::Comment("comment".to_string());
        assert_ne!(line1, line2);
    }

    #[test]
    fn test_is_comment() {
        let line = Line::Comment("comment".to_string());
        assert!(line.is_comment());
    }

    #[test]
    fn test_is_variable() {
        let line = Line::Variable(Variable::new("KEY", "value"));
        assert!(line.is_variable());
    }
}
