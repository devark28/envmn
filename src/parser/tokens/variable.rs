use crate::parser::tokens::token_name::TokenName;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub key: String,
    pub value: String,
}

impl Variable {
    pub fn new(key: &str, value: &str) -> Self {
        Variable {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}={1}", self.key, self.value)
    }
}

impl TokenName for Variable {
    fn name() -> &'static str {
        "Variable"
    }
}
