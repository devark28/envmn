use crate::cli::{Source, args::{Args, Commands as ArgCommands}};
use crate::cli::constants::DEFAULT_FILE;
use crate::error::{CliErrors, Error};

#[derive(Clone, Debug)]
pub struct Cli {
    pub input: Option<Source>,
    pub command: Commands,
}

#[derive(Clone, Debug)]
pub enum Commands {
    Version { name: String, version: String },
    Lint,
    Format,
    List,
    Pick { block_name: String },
}

impl Cli {
    pub fn init() -> Result<Self, Error> {
        let (args, stdin_input) = Args::parse_with_stdin();
        
        if args.version {
            return Ok(Cli {
                input: None,
                command: Commands::Version {
                    name: env!("CARGO_PKG_NAME").to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                },
            });
        }
        
        let Some(command) = args.command else {
            return Err(Error::CliError(CliErrors::NoOperationFound));
        };
        
        let (command, input) = match command {

            ArgCommands::Lint { file } => (
                Commands::Lint,
                Some(Self::resolve_input(file, stdin_input))
            ),
            ArgCommands::Format { file } => (
                Commands::Format,
                Some(Self::resolve_input(file, stdin_input))
            ),
            ArgCommands::List { file } => (
                Commands::List,
                Some(Self::resolve_input(file, stdin_input))
            ),
            ArgCommands::Pick { block, file } => (
                Commands::Pick { block_name: block },
                Some(Self::resolve_input(file, stdin_input))
            ),
        };
        
        Ok(Cli { input, command })
    }
    
    fn resolve_input(file: Option<String>, stdin_input: Option<Source>) -> Source {
        if let Some(file_name) = file {
            Source::FileName(file_name)
        } else if let Some(input) = stdin_input {
            input
        } else {
            Source::FileName(DEFAULT_FILE.to_string())
        }
    }
}