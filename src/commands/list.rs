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

}

// Prerequesties: valid path, file must exist, file must be valid
// Output: All stored tasks
pub fn list(args: ListArgs) -> Result<(), ListError> {
    let json_content = fs::read_to_string(&args.path).unwrap();
    let task_store = deserialize_task_store(&json_content).unwrap();

    task_store
        .iter()
        .for_each(|x| println!("[ ] {}. {}", x.id(), x.title()));

    Ok(())
}