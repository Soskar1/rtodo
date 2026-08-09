use clap::Args;
use std::path::PathBuf;
use thiserror::Error;

use crate::{errors::StorageError, task::{TaskError, load_store, save_store}};

#[derive(Args)]
pub struct DoneArgs {
    path: PathBuf,
    id: u64
}

#[derive(Debug, Error)]
pub enum DoneError {
    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Task(#[from] TaskError)
}

pub fn done(args: DoneArgs) -> Result<(), DoneError> {
    let mut task_store = load_store(&args.path)?;

    let task = task_store.complete_task(&args.id)?;

    println!("Completed task {}: {}", &args.id, task.title());
    
    save_store(&args.path, &task_store)?;

    Ok(())
}