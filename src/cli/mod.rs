mod commands;
mod constants;
mod source;

use crate::error::{CliErrors, Error};
use commands::cli_cmd::CliCmd;
pub use commands::format::FormatCmd;
pub use commands::lint::LintCmd;
pub use commands::list::ListCmd;
pub use commands::pick::PickCmd;
pub use commands::version::VersionCmd;
pub use commands::help::HelpCmd;
pub use commands::Commands;
pub use source::Source;
use std::env;
use std::io::{stdin, IsTerminal, Read};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Cli {
    pub input: Option<Source>,
    pub command: Option<Commands>,
}

impl Cli {
    pub fn init() -> Result<Self, Error> {
        let params = env::args().skip(1).collect::<Vec<_>>();
        let stdin_input = {
            let mut buffer = String::new();
            if !stdin().is_terminal() {
                match stdin().read_to_string(&mut buffer) {
                    Ok(_) => Some(Source::StdIn(buffer)),
                    Err(_) => None,
                }
            } else {
                None
            }
        };
        if stdin_input.is_some() && params.is_empty() {
            return Err(Error::CliError(CliErrors::NoOperationFound));
        }
        if let Some(cmd) = VersionCmd::try_from(params.deref())? {
            <Cli as CliCmd<VersionCmd>>::try_from(cmd, stdin_input)
        } else if let Some(cmd) = HelpCmd::try_from(params.deref())? {
            <Cli as CliCmd<HelpCmd>>::try_from(cmd, stdin_input)
        } else if let Some(cmd) = LintCmd::try_from(params.deref())? {
            <Cli as CliCmd<LintCmd>>::try_from(cmd, stdin_input)
        } else if let Some(cmd) = FormatCmd::try_from(params.deref())? {
            <Cli as CliCmd<FormatCmd>>::try_from(cmd, stdin_input)
        } else if let Some(cmd) = ListCmd::try_from(params.deref())? {
            <Cli as CliCmd<ListCmd>>::try_from(cmd, stdin_input)
        } else if let Some(cmd) = PickCmd::try_from(params.deref())? {
            <Cli as CliCmd<PickCmd>>::try_from(cmd, stdin_input)
        } else {
            match params.first() {
                Some(param) => Err(Error::CliError(CliErrors::UnknownCommand(
                    param.to_string(),
                ))),
                None => Err(Error::CliError(CliErrors::NoOperationFound)),
            }
        }
    }
}

/*fn test_cmds_gracefully<T>(try_from_test: Result<T, Error>) -> Option<Result<T, Error>> {
    match try_from_test {
        Err(err) => match err {
            Error::CliError(_) => None,
            other_err => Some(Err(other_err)),
        },
        _ => Some(try_from_test),
    }
}*/
