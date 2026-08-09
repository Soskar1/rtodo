use crate::{commands::{RTodoArgs, RTodoError}};

pub mod commands;
mod task;

pub fn run(args: RTodoArgs) -> Result<(), RTodoError> {
   args.execute()
}