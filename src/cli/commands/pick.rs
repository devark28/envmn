use crate::cli::CliCmd;
use crate::cli::constants::DEFAULT_FILE;
use crate::cli::{Cli, Commands, Source};
use crate::error::{CliErrors, Error};

const COMMAND_NAME: &str = "pick";

#[derive(Clone, Debug)]
pub struct PickCmd {
    pub block_name: String,
    file_name: Option<String>,
}

impl PickCmd {
    pub fn try_from(params: &[String]) -> Result<Option<Self>, Error> {
        let mut params_iter = params.iter();
        let cmd_token = match params_iter.next() {
            Some(cmd_name) if cmd_name == COMMAND_NAME => Some(cmd_name),
            _ => None,
        };
        if cmd_token.is_none() {
            return Ok(None);
        }
        let (_, block_name) = match params_iter.next() {
            Some(block_name) => (cmd_token.unwrap(), Some(block_name)),
            None => (cmd_token.unwrap(), None),
        };
        if block_name.is_none() {
            return Err(Error::CliError(CliErrors::FailedToParseArgs(
                "Provide a block name",
            )));
        }
        let (block_name, file_name) = match params_iter.next() {
            Some(file_name) => (block_name.unwrap(), Some(file_name)),
            None => (block_name.unwrap(), None),
        };
        match file_name {
            Some(file_name) => Ok(Some(PickCmd {
                block_name: block_name.to_string(),
                file_name: Some(file_name.to_string()),
            })),
            None => Ok(Some(PickCmd {
                block_name: block_name.to_string(),
                file_name: None,
            })),
        }
    }
}

impl CliCmd<PickCmd> for Cli {
    fn try_from(cmd: PickCmd, stdin_input: Option<Source>) -> Result<Self, Error> {
        Ok(Cli {
            input: {
                if let Some(file_name) = cmd.clone().file_name {
                    Some(Source::FileName(file_name.to_string()))
                } else if let Some(input) = stdin_input {
                    Some(input)
                } else {
                    Some(Source::FileName(DEFAULT_FILE.to_string()))
                }
            },
            command: Some(Commands::PickCmd(cmd)),
        })
    }
}

/*impl TryInto<Cli> for PickCmd {
    type Error = Box<dyn Error>;
    fn try_into(self) -> Result<Cli, Self::Error> {
        Ok(Cli {
            input: if let Some(file_name) = self.clone().file_name {
                Some(InputType::FileName(file_name.to_string()))
            } else if let Some(input) = stdin_input() {
                Some(input)
            } else {
                Some(InputType::FileName(DEFAULT_FILE.to_string()))
            },
            command: Some(CommandType::PickCmd(self)),
        })
    }
}*/
