use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::{commands::add::AddError};

pub mod add;

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
    Add(add::AddArgs)
}

#[derive(Debug, Error)]
pub enum RTodoError {
    #[error(transparent)]
    Add(#[from] AddError)
}

impl Commands {
    fn execute(self) -> Result<(), RTodoError> {
        match self {
            Self::Add(args) => add::add(args)?
        }

        Ok(())
    }
}