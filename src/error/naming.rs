use std::fmt::{Display, Formatter};

#[allow(unused)]
#[derive(Debug)]
pub enum NamingErrors {
    NameEmpty(&'static str),
    ContainsInvalidCharacter(u16, String, &'static str),
    StartsWithInvalidCharacter(u16, String),
}

impl Display for NamingErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NamingErrors::NameEmpty(token) => {
                write!(f, "{token} name can not be empty")
            }
            NamingErrors::ContainsInvalidCharacter(line, invalid_char, token) => {
                write!(
                    f,
                    "Line {0}: {token} name contains invalid characters '{invalid_char}'",
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
