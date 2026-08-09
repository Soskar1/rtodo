use crate::{commands::{RTodoArgs, RTodoError}};

pub mod commands;
mod task;
mod errors;

#[cfg(test)]
pub(crate) mod test_helpers;

pub fn run(args: RTodoArgs) -> Result<(), RTodoError> {
   args.execute()
}