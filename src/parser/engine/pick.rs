use crate::cli::Source;
use crate::error::CliErrors;
use crate::parser::engine::Engine;
use std::fs;
use std::process::exit;

impl Engine {
    pub fn process_pick_cmd(mut self, block_name: String, tags: Vec<String>) {
        let active_before = self.document.active_by_tag();
        let picked_tagged_block = self
            .document
            .find_indices(block_name.as_str(), &tags)
            .iter()
            .any(|&index| {
                self.document.get_blocks()[index]
                    .resource_tags()
                    .next()
                    .is_some()
            });
        match self.document.pick(block_name.as_str(), &tags) {
            Ok(document) => {
                let mut changed = false;
                for (tag, active) in document.active_by_tag() {
                    match active_before.get(&tag) {
                        Some(previous) if *previous != active => {
                            eprintln!("{tag}: '{active}' now active (was '{previous}')");
                            changed = true;
                        }
                        _ => (),
                    }
                }
                if !changed && picked_tagged_block {
                    eprintln!("'{block_name}' is already active");
                }
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
