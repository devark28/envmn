use crate::cli::cli_parser::CliCmd;
use crate::cli::constants::DEFAULT_FILE;
use crate::cli::{Cli, Command, InputSource};
use crate::error::Error;

const COMMAND_NAME: &str = "list";

#[derive(Clone, Debug)]
pub struct ListCmd {
    file_name: Option<String>,
}

impl ListCmd {
    pub fn try_from(params: &[String]) -> Result<Option<Self>, Error> {
        let mut params_iter = params.iter();
        let cmd_token = match params_iter.next() {
            Some(cmd_name) if cmd_name == COMMAND_NAME => Some(cmd_name),
            _ => None,
        };
        if cmd_token.is_none() {
            return Ok(None);
        }
        let (_, file_name) = match params_iter.next() {
            Some(file_name) => (cmd_token.unwrap(), Some(file_name)),
            None => (cmd_token.unwrap(), None),
        };
        match file_name {
            Some(file_name) => Ok(Some(ListCmd {
                file_name: Some(file_name.to_string()),
            })),
            None => Ok(Some(ListCmd { file_name: None })),
        }
    }
}

impl CliCmd<ListCmd> for Cli {
    fn try_from(cmd: ListCmd, stdin_input: Option<InputSource>) -> Result<Self, Error> {
        Ok(Cli {
            input: {
                if let Some(file_name) = cmd.clone().file_name {
                    Some(InputSource::FileName(file_name.to_string()))
                } else if let Some(input) = stdin_input {
                    Some(input)
                } else {
                    Some(InputSource::FileName(DEFAULT_FILE.to_string()))
                }
            },
            command: Some(Command::ListCmd),
        })
    }
}
