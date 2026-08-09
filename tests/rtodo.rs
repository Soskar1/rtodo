use assert_cmd::Command;
use predicates::prelude::predicate;
use rstest::rstest;
use tempfile::{TempDir, tempdir};
use std::path::{Path, PathBuf};
use std::fs;

const RTODO: &str = "rtodo";

fn create_temp_store() -> (TempDir, PathBuf) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("todo.txt");

    (directory, path)
}

fn add_command(path: &Path, task_name: &str) -> Command {
    let mut command = Command::cargo_bin(RTODO).unwrap();

    command
        .arg("add")
        .arg(path)
        .arg(task_name);

    command
}

fn list_command(path: &Path) -> Command {
    let mut command = Command::cargo_bin(RTODO).unwrap();

    command
        .arg("list")
        .arg(path);

    command
}

#[rstest]
#[case("Learn Rust")]
#[case("Learn ownership")]
fn add(#[case] task_name: &str) {
    let (_directory, path) = create_temp_store();

    add_command(&path, task_name)
        .assert()
        .success()
        .stdout(predicate::eq(format!("Added task 1: {}\n", task_name)));
}

#[test]
fn add_fails_when_task_title_is_empty() {
    let (_directory, path) = create_temp_store();

    add_command(&path, "")
        .assert()
        .failure()
        .stderr(predicate::str::contains("task title cannot be empty"));
}

#[test]
fn add_fails_with_invalid_path() {
    let path = PathBuf::from("/<>:\"\\|?*");

    add_command(&path, "task1")
        .assert()
        .failure()
        .stderr(predicate::str::contains("I/O error:"));
}

#[test]
fn add_invalid_data_file_content_error() {
    let (_directory, path) = create_temp_store();

    fs::write(&path, "Hello").unwrap();

    add_command(&path, "test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid data file content:"));
}

#[test]
fn add_mutliple_valid_tasks() {
    let (_directory, path) = create_temp_store();

    add_command(&path, "task1")
        .assert()
        .success();

    add_command(&path, "task2")
        .assert()
        .success()
        .stdout("Added task 2: task2\n");
}

#[test]
fn list_prints_task() {
    let (_directory, path) = create_temp_store();

    add_command(&path, "task1")
        .assert()
        .success();

    list_command(&path)
        .assert()
        .success()
        .stdout("[ ] 1. task1\n");
}