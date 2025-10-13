mod access;
mod cli;
mod naming;
mod parsing;

pub use access::AccessErrors;
pub use cli::CliErrors;
pub use naming::NamingErrors;
pub use parsing::ParsingErrors;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    AccessError(AccessErrors),
    NamingError(NamingErrors),
    ParsingError(ParsingErrors),
    CliError(CliErrors),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::AccessError(err) => err.to_string(),
                Error::NamingError(err) => err.to_string(),
                Error::ParsingError(err) => err.to_string(),
                Error::CliError(err) => err.to_string(),
            }
        )
    }
}
