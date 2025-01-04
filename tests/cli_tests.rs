use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("create-cursor-app").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("ARGS:"))
        .stdout(predicate::str::contains("OPTIONS:"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("create-cursor-app").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_missing_directory() {
    let mut cmd = Command::cargo_bin("create-cursor-app").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_cli_with_directory() {
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("test-project");

    // Create a mock templates directory
    let templates_dir = temp_dir.path().join("templates/basic");
    fs::create_dir_all(&templates_dir).unwrap();

    // Create required template files
    fs::write(
        templates_dir.join("PROMPT.md"),
        "# {{project_name}}",
    ).unwrap();
    fs::write(
        templates_dir.join("CHANGELOG.md"),
        "# Changelog for {{project_name}}",
    ).unwrap();
    fs::write(
        templates_dir.join("PROJECT_SCOPE.md"),
        "# Scope for {{project_name}}",
    ).unwrap();
    fs::write(
        templates_dir.join(".cursorrules"),
        r#"{
            "settings": {
                "context_tracking": {
                    "enabled": true,
                    "files": ["PROMPT.md", "CHANGELOG.md", "PROJECT_SCOPE.md"]
                }
            }
        }"#,
    ).unwrap();

    // Set up environment for test
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let mut cmd = Command::cargo_bin("create-cursor-app").unwrap();
    cmd.arg(output_dir.to_str().unwrap())
        .env("RUST_BACKTRACE", "1")
        .env("TEST_MODE", "1") // We'll use this to bypass interactive prompts in test mode
        .assert()
        .success()
        .stdout(predicate::str::contains("Project created successfully!"));

    // Verify files were created
    assert!(output_dir.join("PROMPT.md").exists());
    assert!(output_dir.join("CHANGELOG.md").exists());
    assert!(output_dir.join("PROJECT_SCOPE.md").exists());
} 