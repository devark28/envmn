mod common;

use common::{create_test_env_file};

#[test]
fn list_command_with_file() {
    let test_content = r#"# Basic config
API_URL=https://api.example.com

#@ database_block
DB_HOST=localhost
DB_PORT=5432
##

#@ api_block
API_KEY=secret
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = std::process::Command::new(common::get_binary_path())
        .arg("list")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("database_block"));
    assert!(stdout.contains("api_block"));
}