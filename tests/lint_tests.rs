mod common;

use common::create_test_env_file;

#[test]
fn lint_warns_on_asymmetric_group() {
    let test_content = r#"#@ local [db]
DB_HOST=localhost
DB_PORT=5432
##

#@ remote [db]
DB_HOST=example.com
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = std::process::Command::new(common::get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    // symmetry findings are advisory: warn on stderr, still exit 0
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("warning: 'remote [db]' is missing variable 'DB_PORT'"));
}

#[test]
fn lint_symmetric_tagged_file_is_clean() {
    let test_content = r#"#@ local [db]
DB_HOST=localhost
##

#@ remote [db]
DB_HOST=example.com
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = std::process::Command::new(common::get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}

#[test]
fn lint_checks_encrypted_blocks_by_their_plain_keys() {
    let test_content = r#"#@ local [db]
DB_HOST=localhost
DB_PASSWORD=hunter2
##

#@ remote [db, __encrypted__]
DB_HOST=08debe3d42ade91671f783a784fbed31dd3e897abae5439be07f885887217136
##
"#;

    let temp_file = create_test_env_file(test_content);

    let output = std::process::Command::new(common::get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("warning: 'remote [db, __encrypted__]' is missing variable 'DB_PASSWORD'")
    );
}

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
