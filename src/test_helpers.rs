use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

pub struct TestStore {
    _directory: TempDir,
    pub path: PathBuf
}

impl TestStore {
    pub fn new() -> Self {
        let directory = tempdir().unwrap();
        let path = directory.path().join("todo.txt");

        Self {
            _directory: directory,
            path
        }
    }
}