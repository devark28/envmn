mod cli;
mod error;
mod parser;

use std::fs;
use crate::cli::{Cli, Command, InputSource};
use crate::error::{AccessErrors, CliErrors, OtherErrors};
use crate::parser::Parser;
use std::process::exit;

fn main() {
    let cli = match Cli::init() {
        Ok(cli) => cli,
        Err(error_type) => {
            eprintln!("{}", error_type);
            exit(1);
        }
    };
    let parser = Parser::new();
    let document = match &cli.input {
        Some(InputSource::StdIn(content)) => parser.parse(content),
        Some(InputSource::FileName(name)) => parser.parse_file(name),
        _ => {
            eprintln!("{}", CliErrors::NoInputFound);
            exit(1);
        }
    };
    if let Some(command) = cli.command {
        match document {
            Ok(mut document) => match command {
                Command::PickCmd(pick_cmd) => {
                    match document.get_block(pick_cmd.block_name.as_str()) {
                        Some(_) => {
                            match document.pick(pick_cmd.block_name.as_str()) {
                                Ok(document) => {
                                    match &cli.input {
                                        Some(InputSource::StdIn(_)) => {
                                            print!("{}", document);
                                        },
                                        Some(InputSource::FileName(file_path)) => {
                                            let content = format!("{}", document);
                                            match fs::write(file_path, content) {
                                                Ok(_) => (),
                                                Err(_) => {
                                                    eprintln!("{}", OtherErrors::Unknown);
                                                    exit(1);
                                                },
                                            };
                                        },
                                        _ => {
                                            eprintln!("{}", CliErrors::NoInputFound);
                                            exit(1);
                                        }
                                    };
                                }
                                Err(error_type) => {
                                    eprintln!("{}", error_type);
                                    exit(1);
                                }
                            };
                        }
                        None => {
                            eprintln!("{}", AccessErrors::BlockNotFound(pick_cmd.block_name));
                            exit(1);
                        }
                    }
                }
                Command::ListCmd => {
                    println!(
                        "Blocks ({}):\n{}",
                        document.len(),
                        document
                            .get_blocks()
                            .iter()
                            .map(|b| format!("- {}", b.name.clone()))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                }
            },
            Err(error_type) => {
                eprintln!("{}", error_type);
                exit(1);
            }
        }
    } else {
        eprintln!("{}", CliErrors::NoOperationFound);
        exit(1);
    }
}
