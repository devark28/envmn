use crate::parser::engine::Engine;

impl Engine {
    pub fn process_help_cmd() {
        println!("
envmn — environment manager for .env-style files

Usage:
  envmn <command> [options] [file]

Commands:
  help                  Show this help message
  version               Display the current version
  list                  List all environment blocks in the file
  lint                  Check for syntax and linting errors
  format                Pretty-format the file
  pick <block>          Reorder the file by moving the specified block down

Input modes:
  - If data is piped in, envmn reads from standard input and writes to standard output.
  - If both a pipe and a file are provided, the piped input takes priority.
  - If no file is provided, envmn assumes a `.env` file exists in the current directory (for convenience).
  - When a file path is provided (or .env is assumed), envmn reads from (and edits, if a file was passed) the file directly.

Examples:
  cat .env | envmn lint
  envmn format .env
  envmn pick database_block .env > out.env
  envmn version

For more information, visit: https://github.com/devark28/envmn
");
    }
}
