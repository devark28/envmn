use crate::cli::CliCmd;
use crate::cli::{Commands, FormatCmd, HelpCmd, LintCmd, ListCmd, PickCmd, Source, VersionCmd};
use crate::error::{CliErrors, Error};
use crate::try_parse_cmd;
use std::env;
use std::io::{IsTerminal, Read, stdin};
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
        try_parse_cmd!(VersionCmd, params.deref(), stdin_input);
        try_parse_cmd!(HelpCmd, params.deref(), stdin_input);
        try_parse_cmd!(LintCmd, params.deref(), stdin_input);
        try_parse_cmd!(FormatCmd, params.deref(), stdin_input);
        try_parse_cmd!(ListCmd, params.deref(), stdin_input);
        try_parse_cmd!(PickCmd, params.deref(), stdin_input);
        match params.first() {
            Some(param) => Err(Error::CliError(CliErrors::UnknownCommand(
                param.to_string(),
            ))),
            None => Err(Error::CliError(CliErrors::NoOperationFound)),
        }
    }
}
