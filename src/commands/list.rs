use clap::Args;
use thiserror::Error;
use std::path::PathBuf;

use crate::{errors::StorageError, task::load_store};

#[derive(Args)]
pub struct ListArgs {
    path: PathBuf
}

#[derive(Debug, Error)]
pub enum ListError {
   #[error(transparent)]
    Storage(#[from] StorageError)
}

// Prerequesties: valid path, file must exist, file must be valid
// Output: All stored tasks
pub fn list(args: ListArgs) -> Result<(), ListError> {
    let task_store = load_store(&args.path)?;

    if task_store.size() == 0 {
        println!("No tasks found.");
        return Ok(());
    }

    task_store
        .iter()
        .for_each(|x| println!("{}", x));

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
            ListError::Storage(StorageError::Io(_))
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
            ListError::Storage(StorageError::Io(_))
        ))
    }
}