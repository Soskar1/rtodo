use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::commands::{add::AddError, done::DoneError, list::ListError};
use crate::commands::remove::RemoveError;

mod add;
mod list;
mod done;
mod remove;

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
    List(list::ListArgs),
    Done(done::DoneArgs),
    Remove(remove::RemoveArgs)
}

#[derive(Debug, Error)]
pub enum RTodoError {
    #[error(transparent)]
    Add(#[from] AddError),

    #[error(transparent)]
    List(#[from] ListError),

    #[error(transparent)]
    Done(#[from] DoneError),
    
    #[error(transparent)]
    Remove(#[from] RemoveError),
}

impl Commands {
    fn execute(self) -> Result<(), RTodoError> {
        match self {
            Self::Add(args) => add::add(args)?,
            Self::List(args) => list::list(args)?,
            Self::Done(args) => done::done(args)?,
            Self::Remove(args) => remove::remove(args)?
        }

        Ok(())
    }
}