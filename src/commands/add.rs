use std::path::PathBuf;
use clap::Args;
use crate::{errors::StorageError, task::{TaskError::{self, EmptyTitle}, TaskStore, load_store, save_store}};
use thiserror::Error;

#[derive(Args)]
pub struct AddArgs {
    path: PathBuf,
    task_name: String
}

#[derive(Debug, Error)]
pub enum AddError {
    #[error(transparent)]
    Task(#[from] TaskError),

    #[error(transparent)]
    Storage(#[from] StorageError)
}

pub fn add(args: AddArgs) -> Result<(), AddError> {
    if args.task_name.is_empty() {
        return Err(AddError::Task(EmptyTitle));
    }

    let mut task_store = if args.path.exists() {
        load_store(&args.path)?
    } else {
        TaskStore::default()
    };

    task_store.add(&args.task_name)?;
    println!("Added task {}: {}", task_store.size(), &args.task_name);

    save_store(&args.path, &task_store)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::test_helpers::TestStore;
    use std::fs;

    #[rstest]
    #[case("Learn Rust")]
    #[case("Learn ownership")]
    fn add_task(#[case] task_name: &str) {
        // Arrange
        let store = TestStore::new();
        let path = store.path;

        // Act
        let result = add(AddArgs {
            path: path.clone(),
            task_name: task_name.to_string()
        });

        // Assert
        assert!(result.is_ok());

        let json = fs::read_to_string(&path).unwrap();
        let store: TaskStore = serde_json::from_str(&json).unwrap();

        assert_eq!(store.size(), 1);

        let task = store.get_task(1).unwrap();

        assert_eq!(task.id(), 1);
        assert_eq!(task.title(), task_name);
        assert_eq!(task.completed(), false);
    }

    #[test]
    fn add_empty_title_task_is_not_allowed() {
        // Arrange
        let store = TestStore::new();
        let path = store.path;

        // Act
        let result = add(AddArgs {
            path: path.clone(),
            task_name: "".to_string()
        });

        // Assert
        assert!(result.is_err());

        let error = result.unwrap_err();

        assert!(matches!(
            error,
            AddError::Task(TaskError::EmptyTitle)
        ))
    }

    #[test]
    fn add_invalid_path_is_not_allowed() {
        // Arrange
        let path = PathBuf::from("/<>:\"\\|?*");

        // Act
        let result = add(AddArgs {
            path,
            task_name: "test".to_string()
        });

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(matches!(
            error,
            AddError::Storage(StorageError::Io(_))
        ))
    }

    #[test]
    fn add_fails_with_invalid_file_data() {
        // Arrange
        let store = TestStore::new();
        let path = store.path;

        fs::write(&path, "Hello").unwrap();

        // Act
        let result = add(AddArgs {
            path,
            task_name: "test".to_string()
        });

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(matches!(
            error,
            AddError::Storage(StorageError::Json(_))
        ))
    }

    #[test]
    fn add_task_count_increments() {
        // Arrange
        let store = TestStore::new();
        let path = store.path;

        let _ = add(AddArgs {
            path: path.clone(),
            task_name: "task1".to_string()
        });

        // Act
        let result = add(AddArgs {
            path: path.clone(),
            task_name: "task2".to_string()
        });

        // Assert
        assert!(result.is_ok());

        let json = fs::read_to_string(&path).unwrap();
        let store: TaskStore = serde_json::from_str(&json).unwrap();

        assert_eq!(store.size(), 2);

        let task = store.get_task(2).unwrap();

        assert_eq!(task.id(), 2);
        assert_eq!(task.title(), "task2");
        assert_eq!(task.completed(), false);
    }
}

