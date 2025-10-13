use crate::cli::VersionCmd;
use crate::parser::engine::Engine;

impl Engine {
    pub fn process_version_cmd(&self, version_cmd: VersionCmd) {
        println!("{0} version {1}", version_cmd.name, version_cmd.version);
    }
}
