mod common;

use common::run_command;

#[test]
fn version_command() {
    let output = run_command(&["version"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("envmn version"));
    assert!(stdout.contains("0.2.7"));
}

#[test]
fn version_flag() {
    let output = run_command(&["--version"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("envmn version"));
}

#[test]
fn no_command_shows_help() {
    let output = run_command(&[]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stderr.contains("Environment manager") || stdout.contains("Environment manager"));
}

#[test]
fn invalid_command() {
    let output = run_command(&["invalid_command"]);
    assert!(!output.status.success());
}
