use assert_cmd::Command;
use predicates::prelude::predicate;
use rstest::rstest;
use tempfile::tempdir;
use std::path::PathBuf;

#[rstest]
#[case("Learn Rust")]
#[case("Learn ownership")]
fn add(#[case] task_name: &str) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("todo.txt");

    Command::cargo_bin("rtodo")
        .unwrap()
        .arg("add")
        .arg(&path)
        .arg(task_name)
        .assert()
        .success()
        .stdout(predicate::eq(format!("Added task 1: {}\n", task_name)));
}

#[test]
fn add_fails_when_task_title_is_empty() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("todo.txt");

    Command::cargo_bin("rtodo")
        .unwrap()
        .arg("add")
        .arg(&path)
        .arg("")
        .assert()
        .failure()
        .stderr(predicate::str::contains("task title cannot be empty"));
}

#[test]
fn add_fails_with_invalid_path() {
    let path = PathBuf::from("/<>:\"\\|?*");

    Command::cargo_bin("rtodo")
        .unwrap()
        .arg("add")
        .arg(&path)
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("I/O error:"));
}