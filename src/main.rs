mod cli;
mod error;
mod parser;

use crate::cli::{Cli, Commands, Source};
use crate::error::CliErrors;
use crate::parser::{Engine, Parser};
use std::process::exit;

fn main() {
    let cli = match Cli::init() {
        Ok(cli) => cli,
        Err(error_type) => {
            eprintln!("{}", error_type);
            exit(1);
        }
    };
    match &cli.command {
        Some(Commands::VersionCmd(version_cmd)) => {
            Engine::process_version_cmd(version_cmd.clone());
            exit(0);
        }
        _ => (),
    };
    let parser = Parser::new();
    let Some(source) = &cli.input else {
        eprintln!("{}", CliErrors::NoInputFound);
        exit(1);
    };
    let result = match source {
        Source::StdIn(content) => parser.parse(content),
        Source::FileName(name) => parser.parse_file(name),
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
