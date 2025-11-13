use crate::cli::Source;
use crate::error::CliErrors;
use crate::parser::engine::Engine;
use std::fs;
use std::process::exit;

impl Engine {
    pub fn process_format_cmd(self) {
        let Some(input) = &self.cli.input else {
            eprintln!("{}", CliErrors::NoInputFound);
            exit(1);
        };
        let Source::FileName(file_path) = input else {
            print!("{}", self.document);
            exit(0);
        };
        let content = format!("{}", self.document);
        match fs::write(file_path, content) {
            Ok(_) => (),
            Err(error_type) => {
                eprintln!("{}", error_type);
                exit(1);
            }
        };
    }
}
