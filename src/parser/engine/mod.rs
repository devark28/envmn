mod format;
mod lint;
mod list;
mod pick;
mod version;

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
            Commands::Lint => Ok(self.process_lint_cmd()),
            Commands::List => Ok(self.process_list_cmd()),
            Commands::Format => Ok(self.process_format_cmd()),
            Commands::Pick { block_name, tags } => Ok(self.process_pick_cmd(block_name, tags)),
            _ => Err(Error::CliError(CliErrors::NoOperationFound)),
        }
    }
}
