mod commands;

use crate::cli::{Cli, Commands};
use crate::error::{CliErrors, Error};
use crate::parser::tokens::Document;

pub struct Engine {
    cli: Cli,
    document: Document,
}

impl Engine {
    pub fn new(cli: Cli, document: Document) -> Self {
        Engine { cli, document }
    }
    pub fn process(self) -> Result<(), Error> {
        match self.cli.command.clone() {
            Some(Commands::LintCmd) => Ok(()),
            Some(Commands::ListCmd) => Ok(self.process_list_cmd()),
            Some(Commands::FormatCmd) => Ok(self.process_format_cmd()),
            Some(Commands::PickCmd(pick_cmd)) => Ok(self.process_pick_cmd(pick_cmd)),
            Some(Commands::VersionCmd(version_cmd)) => Ok(self.process_version_cmd(version_cmd)),
            None => Err(Error::CliError(CliErrors::NoOperationFound)),
        }
    }
}
