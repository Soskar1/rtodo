use clap::Args;
use thiserror::Error;
use std::{fs, path::PathBuf};

use crate::task::deserialize_task_store;

#[derive(Args)]
pub struct ListArgs {
    path: PathBuf
}

#[derive(Debug, Error)]
pub enum ListError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid data file content: {0}")]
    Json(#[from] serde_json::Error)
}

// Prerequesties: valid path, file must exist, file must be valid
// Output: All stored tasks
pub fn list(args: ListArgs) -> Result<(), ListError> {
    let json_content = fs::read_to_string(&args.path)?;
    let task_store = deserialize_task_store(&json_content)?;

    if task_store.size() == 0 {
        println!("No tasks found.");
        return Ok(());
    }

    task_store
        .iter()
        .for_each(|x| println!("[ ] {}. {}", x.id(), x.title()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_invalid_path_error() {
        // Arrange
        let path = PathBuf::from("/<>:\"\\|?*");

        // Act
        let result = list(ListArgs {
            path
        });

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(matches!(
            error,
            ListError::Io(_)
        ))
    }

    #[test]
    fn list_file_does_not_exist_error() {
        // Arrange
        let path = PathBuf::from("todo.json");

        // Act
        let result = list(ListArgs {
            path
        });

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(matches!(
            error,
            ListError::Io(_)
        ))
    }
}