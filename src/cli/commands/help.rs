use crate::cli::CliCmd;
use crate::cli::{Cli, Commands, Source};
use crate::error::Error;

const COMMAND_NAME: &str = "help";

#[derive(Clone, Debug)]
pub struct HelpCmd;

impl HelpCmd {
    pub fn try_from(params: &[String]) -> Result<Option<Self>, Error> {
        let mut params_iter = params.iter();
        let cmd_token = match params_iter.next() {
            Some(cmd_name) if cmd_name == COMMAND_NAME => Some(cmd_name),
            _ => None,
        };
        if cmd_token.is_none() {
            return Ok(None);
        }
        Ok(Some(HelpCmd))
    }
}

impl CliCmd<HelpCmd> for Cli {
    fn try_from(_cmd: HelpCmd, _stdin_input: Option<Source>) -> Result<Self, Error> {
        Ok(Cli {
            input: None,
            command: Some(Commands::HelpCmd),
        })
    }
}
