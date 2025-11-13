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
            Commands::Lint => Ok(()),
            Commands::List => Ok(self.process_list_cmd()),
            Commands::Format => Ok(self.process_format_cmd()),
            Commands::Pick { block_name } => Ok(self.process_pick_cmd(block_name)),
            _ => Err(Error::CliError(CliErrors::NoOperationFound)),
        }
    }
}
