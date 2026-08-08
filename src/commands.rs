use clap::{Parser, Subcommand};

pub mod add;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct RTodoArgs {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Add(add::AddArgs)
}

impl RTodoArgs {
    pub fn execute(self) {
        self.command.execute();
    }
}

impl Commands {
    fn execute(self) {
        match self {
            Self::Add(args) => add::add(args)
        }
    }
}