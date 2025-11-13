use crate::parser::engine::Engine;

impl Engine {
    pub fn process_version_cmd(name: &str, version: &str) {
        println!("{0} version {1}", name, version);
    }
}
