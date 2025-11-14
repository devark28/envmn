use clap::{Parser, Subcommand};
use crate::cli::Source;
use std::io::{IsTerminal, Read, stdin};

#[derive(Parser)]
#[command(name = "envmn")]
#[command(about = "Environment manager for .env-style files")]
#[command(after_help = "Input modes:
  - If data is piped in, envmn reads from standard input and writes to standard output.
  - If both a pipe and a file are provided, the piped input takes priority.
  - If no file is provided, envmn assumes a `.env` file exists in the current directory (for convenience).
  - When a file path is provided (or .env is assumed), envmn reads from (and edits, if a file was passed) the file directly.

Examples:
  cat .env | envmn lint
  envmn format .env
  envmn pick database_block .env > out.env
  envmn --version

For more information, visit: https://github.com/devark28/envmn")]
pub struct Args {
    /// Display the current version
    #[arg(short, long)]
    pub version: bool,
    
    #[command(subcommand)]
    pub command: Option<ArgCommands>,
}

#[derive(Subcommand)]
pub enum ArgCommands {
    /// Check for syntax and linting errors
    Lint {
        /// File to lint (defaults to .env)
        file: Option<String>,
    },
    /// Pretty-format the file
    Format {
        /// File to format (defaults to .env)
        file: Option<String>,
    },
    /// List all environment blocks in the file
    List {
        /// File to list blocks from (defaults to .env)
        file: Option<String>,
    },
    /// Reorder the file by moving the specified block down
    Pick {
        /// Block name to move
        block: String,
        /// File to modify (defaults to .env)
        file: Option<String>,
    },
}

impl Args {
    pub fn parse_with_stdin() -> (Self, Option<Source>) {
        let stdin_input = {
            let mut buffer = String::new();
            if !stdin().is_terminal() {
                match stdin().read_to_string(&mut buffer) {
                    Ok(_) => Some(Source::StdIn(buffer)),
                    Err(_) => None,
                }
            } else {
                None
            }
        };
        (Self::parse(), stdin_input)
    }
}