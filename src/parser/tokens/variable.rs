use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
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

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}
