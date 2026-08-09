use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum TaskError {
    #[error("task title cannot be empty")]
    EmptyTitle
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: u64,
    title: String,
    completed: bool
}

impl Task {
    pub fn new(id: u64, title: &str) -> Result<Self, TaskError> {
        if title.is_empty() {
            return Err(TaskError::EmptyTitle);
        }

        Ok(Self {
            id,
            title: String::from(title),
            completed: false
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TaskStore {
    tasks: Vec<Task>,
}

impl TaskStore {
    pub fn add(&mut self, task_title: &str) -> Result<(), TaskError> {
        let task = Task::new(self.size() as u64 + 1, task_title)?;
        self.tasks.push(task);

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_task(&self, id: usize) -> Option<&Task> {
        self.iter().find(|x| x.id() == id as u64)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }
}

pub fn serialize_task_store(store: &TaskStore) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string_pretty(&store)?;
    Ok(json)
}

pub fn deserialize_task_store(json: &str) -> Result<TaskStore, serde_json::Error> {
    let store = serde_json::from_str(&json)?;
    Ok(store)
}

#[cfg(test)]
mod task_tests {
    use super::*;

    #[test]
    fn new_creates_task() {
        // Arrange & Act
        let result = Task::new(0, "hello world");

        // Assert
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.id(), 0);
        assert_eq!(task.title(), "hello world");
        assert_eq!(task.completed(), false);
    }

    #[test]
    fn new_does_not_let_an_empty_title() {
        // Arrange & Act
        let result = Task::new(0, "");

        // Assert
        assert!(!result.is_ok());
        assert_eq!(result.err().unwrap(), TaskError::EmptyTitle);
    }
}

#[cfg(test)]
mod task_store_tests {
    use super::*;
    
    #[test]
    fn add_adds_task() {
        // Arrange
        let mut store = TaskStore::default();
        
        // Act
        let result = store.add("hello world");
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(store.size(), 1);
        
        let task = store.get_task(1).unwrap();
        assert_eq!(task.title(), "hello world");
        assert_eq!(task.completed(), false);
        assert_eq!(task.id(), 1);
    }
}