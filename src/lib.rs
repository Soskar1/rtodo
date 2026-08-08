use crate::commands::RTodoArgs;

pub mod commands;
mod task;

pub fn run(args: RTodoArgs) {
   args.execute();
}