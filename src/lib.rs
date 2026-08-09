use crate::{commands::RTodoArgs, task::TaskError};

pub mod commands;
mod task;

pub fn run(args: RTodoArgs) -> Result<(), TaskError> {
   args.execute()
}