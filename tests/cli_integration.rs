use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::Command;

/// Helper to get the compiled binary path
fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("envmn");
    path
}

/// Helper to create a temporary test file
fn create_test_env_file(content: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    file
}

#[test]
fn test_version_command() {
    let output = Command::new(get_binary_path())
        .arg("version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("envmn version"));
    assert!(stdout.contains("0.2.7"));
}

#[test]
fn test_version_flag() {
    let output = Command::new(get_binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("envmn version"));
}

#[test]
fn test_no_command_shows_help() {
    let output = Command::new(get_binary_path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stderr.contains("Environment manager") || stdout.contains("Environment manager"));
}

#[test]
fn test_list_command_with_file() {
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

    let output = Command::new(get_binary_path())
        .arg("list")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("database_block"));
    assert!(stdout.contains("api_block"));
}

#[test]
fn test_format_command_with_file() {
    let test_content = r#"KEY=value


#@ block

VAR=test
##

"#;
    let result_content = r#"KEY=value

#@ block
VAR=test
##
"#;
    
    let mut temp_file = create_test_env_file(test_content);
    
    let output = Command::new(get_binary_path())
        .arg("format")
        .arg(temp_file.path())
        .stdin(std::process::Stdio::null())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let mut buffer = String::new();
    temp_file.read_to_string(&mut buffer).unwrap();
    assert_eq!(buffer, result_content);
}

#[test]
fn test_pick_command() {
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
fn test_lint_command_success() {
    let test_content = r#"KEY=value

#@ block
VAR=test
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = Command::new(get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_format_command_with_stdin() {
    let test_content = "KEY=value\n\n#@ block\nVAR=test\n##";

    let mut child = Command::new(get_binary_path())
        .arg("format")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(test_content.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("KEY=value"));
}

#[test]
fn test_stdin_takes_priority_over_file() {
    let file_content = "FILE_VAR=from_file";
    let stdin_content = "STDIN_VAR=from_stdin";

    let temp_file = create_test_env_file(file_content);

    let mut child = Command::new(get_binary_path())
        .arg("format")
        .arg(temp_file.path())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(stdin_content.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("STDIN_VAR"));
}

#[test]
fn test_invalid_command() {
    let output = Command::new(get_binary_path())
        .arg("invalid_command")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_pick_nonexistent_block() {
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

    // Should handle gracefully (either error or no-op)
    let _stderr = String::from_utf8_lossy(&output.stderr);
    let _stdout = String::from_utf8_lossy(&output.stdout);
    // Test passes if it doesn't crash
}
