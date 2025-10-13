use std::fmt::{Display, Formatter};

#[allow(unused)]
#[derive(Debug)]
pub enum NamingErrors {
    BlockNameEmpty,
    ContainsInvalidCharacter(u16, String),
    StartsWithInvalidCharacter(u16, String),
}

impl Display for NamingErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NamingErrors::BlockNameEmpty => {
                write!(f, "Block name can not be empty")
            }
            NamingErrors::ContainsInvalidCharacter(line, invalids) => {
                write!(
                    f,
                    "Line {0}: Block name contains invalid characters '{invalids}'",
                    line + 1
                )
            }
            NamingErrors::StartsWithInvalidCharacter(line, invalids) => {
                write!(
                    f,
                    "Line {0}: Block name starts with invalid character '{invalids}'",
                    line + 1
                )
            }
        }
    }
}
