mod common;

use common::{create_test_env_file, get_binary_path};
use std::io::{Read, Write};
use std::process::Command;

#[test]
fn format_command_with_file() {
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
fn format_command_with_stdin() {
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
fn stdin_takes_priority_over_file() {
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