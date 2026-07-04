use crate::parser::tokens::variable::Variable;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash)]
pub enum Line {
    Comment(String),
    Variable(Variable),
    Raw(String),
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Line::Comment(comment) => write!(f, "# {comment}"),
            Line::Variable(variable) => write!(f, "{variable}"),
            Line::Raw(raw) => write!(f, "{raw}"),
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
    fn line_equality_by_variable() {
        let v1 = Variable::new("KEY", "value1");
        let v2 = Variable::new("KEY", "value2");
        let line1 = Line::Variable(v1);
        let line2 = Line::Variable(v2);
        assert_eq!(line1, line2);
    }

    #[test]
    fn line_inequality_by_comment() {
        let line1 = Line::Comment("comment".to_string());
        let line2 = Line::Comment("comment".to_string());
        assert_ne!(line1, line2);
    }

    #[test]
    fn line_inequality_by_raw() {
        let line1 = Line::Raw("08debe3d42ade916".to_string());
        let line2 = Line::Raw("08debe3d42ade916".to_string());
        assert_ne!(line1, line2);
    }

    #[test]
    fn raw_line_displays_verbatim() {
        let line = Line::Raw("08debe3d42ade916".to_string());
        assert_eq!(line.to_string(), "08debe3d42ade916");
    }
}
