use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::Command;

/// Helper to get the compiled binary path
#[allow(unused)]
pub fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("envmn");
    path
}

/// Helper to create a temporary test file
#[allow(unused)]
pub fn create_test_env_file(content: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    file
}

/// Helper to run a command and return output
#[allow(unused)]
pub fn run_command(args: &[&str]) -> std::process::Output {
    Command::new(get_binary_path())
        .args(args)
        .output()
        .expect("Failed to execute command")
}