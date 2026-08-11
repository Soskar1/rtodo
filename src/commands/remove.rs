use std::path::PathBuf;
use clap::Args;
use thiserror::Error;
use crate::commands::done::{DoneArgs, DoneError};
use crate::errors::StorageError;
use crate::task::{load_store, TaskError, save_store};

#[derive(Args)]
pub struct RemoveArgs {
    path: PathBuf,
    id: u64
}

#[derive(Debug, Error)]
pub enum RemoveError {
    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Task(#[from] TaskError)
}

pub fn remove(args: RemoveArgs) -> Result<(), RemoveError> {
    let mut task_store = load_store(&args.path)?;

    let task = task_store.remove_task(args.id)?;

    println!("Removed task {}: {}", &args.id, task.title());

    save_store(&args.path, &task_store)?;

    Ok(())
}