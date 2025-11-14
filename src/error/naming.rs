use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NamingErrors {
    BlockNameEmpty,
    VariableNameEmpty,
    BlockContainsInvalidCharacter(u16, String),
    VariableContainsInvalidCharacter(u16, String),
    StartsWithInvalidCharacter(u16, String),
}

impl Display for NamingErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NamingErrors::BlockNameEmpty => {
                write!(f, "Block name can not be empty")
            }
            NamingErrors::VariableNameEmpty => {
                write!(f, "Variable name can not be empty")
            }
            NamingErrors::BlockContainsInvalidCharacter(line, invalid_char) => {
                write!(
                    f,
                    "Line {0}: Block name contains invalid characters '{invalid_char}'",
                    line + 1
                )
            }
            NamingErrors::VariableContainsInvalidCharacter(line, invalid_char) => {
                write!(
                    f,
                    "Line {0}: Variable name contains invalid characters '{invalid_char}'",
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
