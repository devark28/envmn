use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CliErrors {
    NoOperationFound,
    NoInputFound,
}

impl Display for CliErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliErrors::NoOperationFound => {
                write!(f, "No operation found")
            }
            CliErrors::NoInputFound => {
                write!(f, "No input found")
            }
        }
    }
}
