use crate::cli::CliCmd;
use crate::cli::{Cli, Commands, Source};
use crate::error::Error;

const COMMAND_NAME: &str = "version";

#[derive(Clone, Debug)]
pub struct VersionCmd {
    pub name: String,
    pub version: String,
}

impl VersionCmd {
    pub fn try_from(params: &[String]) -> Result<Option<Self>, Error> {
        let mut params_iter = params.iter();
        let cmd_token = match params_iter.next() {
            Some(cmd_name) if cmd_name == COMMAND_NAME => Some(cmd_name),
            _ => None,
        };
        if cmd_token.is_none() {
            return Ok(None);
        }
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        Ok(Some(VersionCmd {
            name: name.to_string(),
            version: version.to_string(),
        }))
    }
}

impl CliCmd<VersionCmd> for Cli {
    fn try_from(cmd: VersionCmd, _stdin_input: Option<Source>) -> Result<Self, Error> {
        Ok(Cli {
            input: None,
            command: Some(Commands::VersionCmd(cmd)),
        })
    }
}
