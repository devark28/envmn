use crate::cli::PickCmd;
use crate::cli::commands::list::ListCmd;
use crate::error::{CliErrors, Error};
use std::env;
use std::io::{IsTerminal, Read, stdin};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub enum InputSource {
    StdIn(String),
    FileName(String),
}

#[derive(Clone, Debug)]
pub enum Command {
    PickCmd(PickCmd),
    ListCmd,
}

#[derive(Clone, Debug)]
pub struct Cli {
    pub input: Option<InputSource>,
    pub command: Option<Command>,
}

pub trait CliCmd<T> {
    fn try_from(cmd: T, stdin_input: Option<InputSource>) -> Result<Cli, Error>;
}

impl Cli {
    pub fn init() -> Result<Self, Error> {
        let params = env::args().skip(1).collect::<Vec<_>>();
        let stdin_input = stdin_input();
        if stdin_input.is_some() && params.is_empty() {
            return Err(Error::CliError(CliErrors::NoOperationFound));
        }
        if let Some(cmd) = PickCmd::try_from(params.deref())? {
            <Cli as CliCmd<PickCmd>>::try_from(cmd, stdin_input)
        } else if let Some(cmd) = ListCmd::try_from(params.deref())? {
            <Cli as CliCmd<ListCmd>>::try_from(cmd, stdin_input)
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

pub fn stdin_input() -> Option<InputSource> {
    let mut buffer = String::new();
    if !stdin().is_terminal() {
        match stdin().read_to_string(&mut buffer) {
            Ok(_) => Some(InputSource::StdIn(buffer)),
            Err(_) => None,
        }
    } else {
        None
    }
}
