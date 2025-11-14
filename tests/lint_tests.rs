mod common;

use common::create_test_env_file;

#[test]
fn lint_command_with_file() {
    let test_content = r#"KEY=value

#@ block
VAR=test
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = std::process::Command::new(common::get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}