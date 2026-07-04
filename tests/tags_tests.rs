mod common;

use common::{create_test_env_file, get_binary_path};
use std::fs;
use std::process::Command;

const TAGGED_CONTENT: &str = r#"API_URL=https://api.example.com

#@ local [db]
DB_HOST=localhost
DB_PORT=5432
##

#@ remote [db]
DB_HOST=example.com
DB_PORT=5432
##

#@ server
SERVER_PORT="8080"
##

#@ local [smtp, __encrypted__]
08debe3d42ade91671f783a784fbed31dd3e897abae5439be07f885887217136
eeda8bcff5d676460d8ea84bd0e941e4bf5ac466aa21e2512cc1a870e89fb17d
##
"#;

#[test]
fn list_shows_tags() {
    let temp_file = create_test_env_file(TAGGED_CONTENT);

    let output = Command::new(get_binary_path())
        .arg("list")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("- local [db]"));
    assert!(stdout.contains("- remote [db]"));
    assert!(stdout.contains("- server"));
    assert!(stdout.contains("- local [smtp, __encrypted__]"));
}

#[test]
fn pick_with_tag_selects_matching_block() {
    let temp_file = create_test_env_file(TAGGED_CONTENT);

    let output = Command::new(get_binary_path())
        .arg("pick")
        .arg("local")
        .arg("--tag")
        .arg("smtp")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = fs::read_to_string(temp_file.path()).unwrap();
    let smtp_pos = content.find("#@ local [smtp, __encrypted__]").unwrap();
    let db_pos = content.find("#@ local [db]").unwrap();
    let server_pos = content.find("#@ server").unwrap();
    assert!(smtp_pos > db_pos, "picked block should move to the bottom");
    assert!(
        smtp_pos > server_pos,
        "picked block should move to the bottom"
    );
}

#[test]
fn pick_ambiguous_block_errors_with_candidates() {
    let temp_file = create_test_env_file(TAGGED_CONTENT);

    let output = Command::new(get_binary_path())
        .arg("pick")
        .arg("local")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--tag"));
    assert!(stderr.contains("local [db]"));
    assert!(stderr.contains("local [smtp, __encrypted__]"));
}

#[test]
fn pick_with_unknown_tag_errors() {
    let temp_file = create_test_env_file(TAGGED_CONTENT);

    let output = Command::new(get_binary_path())
        .arg("pick")
        .arg("local")
        .arg("--tag")
        .arg("nope")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("was not found"));
}

#[test]
fn pick_unique_tagged_block_by_name_only() {
    let temp_file = create_test_env_file(TAGGED_CONTENT);

    let output = Command::new(get_binary_path())
        .arg("pick")
        .arg("remote")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = fs::read_to_string(temp_file.path()).unwrap();
    let remote_pos = content.find("#@ remote [db]").unwrap();
    let server_pos = content.find("#@ server").unwrap();
    assert!(
        remote_pos > server_pos,
        "picked block should move to the bottom"
    );
}

#[test]
fn format_round_trips_encrypted_block() {
    let temp_file = create_test_env_file(TAGGED_CONTENT);

    let output = Command::new(get_binary_path())
        .arg("format")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = fs::read_to_string(temp_file.path()).unwrap();
    assert!(content.contains("#@ local [smtp, __encrypted__]"));
    assert!(content.contains("08debe3d42ade91671f783a784fbed31dd3e897abae5439be07f885887217136"));
    assert!(content.contains("eeda8bcff5d676460d8ea84bd0e941e4bf5ac466aa21e2512cc1a870e89fb17d"));
}

#[test]
fn malformed_tags_error() {
    let temp_file = create_test_env_file("#@ local [db\nKEY=value\n##\n");

    let output = Command::new(get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.to_lowercase().contains("tags"));
}

#[test]
fn invalid_tag_character_error() {
    let temp_file = create_test_env_file("#@ local [DB]\nKEY=value\n##\n");

    let output = Command::new(get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Tag name"));
}

#[test]
fn unknown_reserved_tag_error() {
    let temp_file = create_test_env_file("#@ local [__secret__]\nKEY=value\n##\n");

    let output = Command::new(get_binary_path())
        .arg("lint")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown reserved tag"));
}
