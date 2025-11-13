use crate::cli::Source;
use crate::error::CliErrors;
use crate::parser::engine::Engine;
use std::fs;
use std::process::exit;

impl Engine {
    pub fn process_pick_cmd(mut self, block_name: String) {
        match self.document.pick(block_name.as_str()) {
            Ok(document) => {
                let Some(input) = &self.cli.input else {
                    eprintln!("{}", CliErrors::NoInputFound);
                    exit(1);
                };
                let Source::FileName(file_path) = input else {
                    print!("{}", document);
                    exit(0);
                };
                let content = format!("{}", document);
                match fs::write(file_path, content) {
                    Ok(_) => (),
                    Err(error_type) => {
                        eprintln!("{}", error_type);
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
}
