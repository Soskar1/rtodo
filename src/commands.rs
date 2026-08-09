use clap::{Parser, Subcommand};

use crate::task::TaskError;

pub mod add;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct RTodoArgs {
    #[command(subcommand)]
    command: Commands
}

impl RTodoArgs {
    pub fn execute(self) -> Result<(), TaskError>{
        self.command.execute()
    }
}

#[derive(Subcommand)]
enum Commands {
    Add(add::AddArgs)
}

impl Commands {
    fn execute(self) -> Result<(), TaskError> {
        match self {
            Self::Add(args) => add::add(args)
        }
    }
}