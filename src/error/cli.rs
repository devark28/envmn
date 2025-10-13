use std::fmt::{Display, Formatter};

#[allow(unused)]
#[derive(Debug)]
pub enum CliErrors {
    UnknownCommand(String),
    NoOperationFound,
    FailedToParseArgs(&'static str),
    NoInputFound,
}

impl Display for CliErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliErrors::UnknownCommand(param_name) => {
                write!(f, "Parameter '{param_name}' is not recognized")
            }
            CliErrors::NoOperationFound => {
                write!(f, "No operation found")
            }
            CliErrors::FailedToParseArgs(message) => {
                write!(f, "{message}")
            }
            CliErrors::NoInputFound => {
                write!(f, "No input found")
            }
        }
    }
}
