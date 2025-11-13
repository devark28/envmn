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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_raw_equality_by_key() {
        let v1 = Variable {
            key: "KEY".to_string(),
            value: "value1".to_string(),
        };
        let v2 = Variable {
            key: "KEY".to_string(),
            value: "value2".to_string(),
        };
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_variable_new_equality_by_key() {
        let v1 = Variable::new("KEY", "value1");
        let v2 = Variable::new("KEY", "value2");
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_raw_and_new_interop() {
        let v1 = Variable {
            key: "KEY".to_string(),
            value: "value1".to_string(),
        };
        let v2 = Variable::new("KEY", "value1");
        assert_eq!(v1, v2);
    }
}
