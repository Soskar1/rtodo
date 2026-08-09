use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::commands::{add::AddError, list::ListError};

pub mod add;
pub mod list;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct RTodoArgs {
    #[command(subcommand)]
    command: Commands
}

impl RTodoArgs {
    pub fn execute(self) -> Result<(), RTodoError>{
        self.command.execute()
    }
}

#[derive(Subcommand)]
enum Commands {
    Add(add::AddArgs),
    List(list::ListArgs)
}

#[derive(Debug, Error)]
pub enum RTodoError {
    #[error(transparent)]
    Add(#[from] AddError),

    #[error(transparent)]
    List(#[from] ListError)
}

impl Commands {
    fn execute(self) -> Result<(), RTodoError> {
        match self {
            Self::Add(args) => add::add(args)?,
            Self::List(args) => list::list(args)?
        }

        Ok(())
    }
}