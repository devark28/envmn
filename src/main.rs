mod cli;
mod error;
mod parser;

use crate::cli::{Cli, Commands, Source};
use crate::error::{CliErrors, Error};
use crate::parser::{Engine, Parser};
use std::process::exit;

fn main() {
    let cli = match Cli::init() {
        Ok(cli) => cli,
        Err(Error::CliError(CliErrors::NoOperationFound)) => {
            Engine::process_help_cmd();
            exit(1);
        }
        Err(error_type) => {
            eprintln!("{}", error_type);
            exit(1);
        }
    };
    let parser = Parser::new();
    let result = match &cli {
        Cli {
            input: None,
            command: Commands::Version { name, version },
        } => {
            Engine::process_version_cmd(name, version);
            exit(0);
        }

        Cli { input: None, .. } => {
            eprintln!("{}", CliErrors::NoInputFound);
            exit(1);
        }
        Cli {
            input: Some(Source::StdIn(content)),
            ..
        } => parser.parse(content),
        Cli {
            input: Some(Source::FileName(name)),
            ..
        } => parser.parse_file(name),
    };
    let document = match result {
        Ok(document) => document,
        Err(error_type) => {
            eprintln!("{}", error_type);
            exit(1);
        }
    };
    let engine = Engine::new(cli, document);
    match engine.process() {
        Err(error_type) => {
            eprintln!("{}", error_type);
            exit(1);
        }
        _ => (),
    }
}
