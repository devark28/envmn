mod common;

use common::{create_test_env_file, get_binary_path};
use std::process::Command;

#[test]
fn pick_command_with_file() {
    let test_content = r#"DEFAULT_VAR=value

#@ database_block
DB_HOST=localhost
##

#@ api_block
API_KEY=secret
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = Command::new(get_binary_path())
        .arg("pick")
        .arg("database_block")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify that database_block appears after api_block in output (moved to bottom)
    if let (Some(db_pos), Some(api_pos)) = (
        stdout.find("#@ database_block"),
        stdout.find("#@ api_block"),
    ) {
        assert!(
            db_pos > api_pos,
            "database_block should come after api_block when picked"
        );
    }
}

#[test]
fn pick_nonexistent_block() {
    let test_content = r#"KEY=value

#@ existing_block
VAR=test
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = Command::new(get_binary_path())
        .arg("pick")
        .arg("nonexistent_block")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");
    assert!(!output.status.success());

    let _stderr = String::from_utf8_lossy(&output.stderr);
    assert!(_stderr.contains("was not found"));
}